use rayon::prelude::*;
use super::*;

use super::Texture;
use std::f32;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{path::PathBuf, thread};
use crossbeam_channel::Sender;
use crossbeam_channel::{Receiver, unbounded};

pub enum LoaderMsg {
    Progress {
        loaded: usize,
        total: usize,
        current: String,
    },
    Image(RawTexture),
    Done,
    Error(String),
}

pub enum LoaderStatus {
    Loading {
        frac: f32,
        current: String
    },
    Done(Image),
    Cancelled,
    Error(String)
}

#[derive(Clone)]
pub struct CancelToken {
    cancelled: std::sync::Arc<AtomicBool>,
}

impl CancelToken {
    pub fn new() -> Self {
        Self {
            cancelled: std::sync::Arc::new(AtomicBool::new(false)),
        }
    }
    
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Relaxed);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }
}

pub struct AsyncTextureLoader {
    rx: Receiver<LoaderMsg>,
    cancel: CancelToken,
    status: LoaderStatus,
    textures: Vec<Texture>,
    loaded: usize
}

impl AsyncTextureLoader {
    pub fn with_folder(path: PathBuf, settings: ProcessSettings) -> Result<Self, String> {
        let mut files = Vec::new();
        let mut folders = Vec::new();

        folders.push(path);

        while let Some(folder) = folders.pop() {
            for path in match folder.read_dir() {
                Ok(iter) => iter,
                Err(err) => return Err(format!("Error reading folder `{}`:\n\n{err}", folder.to_string_lossy()))
            } {
                let path = match path {
                    Ok(path) => path,
                    Err(err) => return Err(format!("Error reading an item in folder `{}`:\n\n{err}", folder.to_string_lossy()))
                };

                let path = path.path();

                if path.is_dir() {
                    folders.push(path);
                } else if path.is_file() {
                    if is_likely_image_file(&path) {
                        files.push(path);
                    }
                } else {
                    return Err(format!("Error reading item `{:?}`, likely due to permission errors.", path))
                }
            }
        }

        Ok(Self::new(files, settings))
    }

    pub fn new(paths: Vec<PathBuf>, process_settings: ProcessSettings) -> Self {
        let (tx, rx) = unbounded();
        let cancel = CancelToken::new();
        let cancel_clone = cancel.clone();

        thread::spawn(move || {
            load_images_parallel(paths, tx, cancel_clone, process_settings.col_sel, Some(process_settings.pixel_size));
        });

        Self {
            rx,
            cancel,
            status: LoaderStatus::Loading { frac: 0.0, current: "Initialising.".to_string() },
            textures: Vec::new(),
            loaded: 0
        }
    }

    pub fn cancel(&mut self) {
        self.status = LoaderStatus::Cancelled;
        self.cancel.cancel();
    }

    pub fn get_status(&mut self, ctx: &mut AppContextHandler) -> &mut LoaderStatus {
        if !matches!(self.status, LoaderStatus::Error(_) | LoaderStatus::Done(_) | LoaderStatus::Cancelled) {
            while let Ok(result) = self.rx.try_recv() {
                match result {
                    LoaderMsg::Progress { loaded, total, current } => {
                        self.loaded += loaded;
                        self.status = LoaderStatus::Loading { frac: self.loaded as f32 / total as f32, current };
                    },
                    LoaderMsg::Image(texture) => self.textures.push(Texture::from_raw(texture)),
                    LoaderMsg::Done => {
                        if self.textures.is_empty() {
                            self.status = LoaderStatus::Error("Couldn't find any valid image files in that folder.".to_string());
                        } else {
                            self.status = LoaderStatus::Done(generate_image(std::mem::take(&mut self.textures), ctx));
                            break;
                        }
                    },
                    LoaderMsg::Error(err) => {
                        self.status = LoaderStatus::Error(err);
                        break;
                    },
                }
            }
        }
        &mut self.status
    }
}

fn generate_image(textures: Vec<Texture>, ctx: &mut AppContextHandler) -> Image {
    let settings = ctx.store.get::<ExportSettings>();
    let pixel_size = settings.process.pixel_size as f32;
    let pixel_int = settings.process.pixel_size;
    let col_sel = settings.process.col_sel;
    let pixels = ctx.store.get::<PixelArray>();
    let rect = settings.place.rect.unwrap_or_else(|| {
        let [WorldPos(x, y), WorldPos(w, h)] = pixels.get_bounds();
        Rect::new(x, y, w - x + 1.0, h - y + 1.0)
    });

    let w = rect.w as u16;
    let h = rect.h as u16;

    let target_w = w as f32 * pixel_size;
    let target_h = h as f32 * pixel_size;

    let render_target = render_target(w as u32 * pixel_int, h as u32 * pixel_int);
    render_target.texture.set_filter(FilterMode::Nearest);
    
    set_camera(&Camera2D {
        target: vec2(target_w / 2.0, target_h / 2.0),
        zoom: vec2(2.0 / target_w, 2.0 / target_h),
        render_target: Some(render_target.clone()),
        ..Default::default()
    });
    
    clear_background(BLANK);

    if settings.place.temperature == 0.0 {
        for pixel in pixels.iter() {
            let x = pixel.pos[0] as f32 - rect.x;
            let y = pixel.pos[1] as f32 - rect.y;
            let col = col_sel.col_from_rgba_arr(pixel.col);
            let mut iter = textures.iter();
            let mut best_texture = iter.next().unwrap();
            let mut best_value = col.distance(best_texture.average);
            for texture in iter {
                let value = col.distance(texture.average);
                if value < best_value {
                    best_texture = texture;
                    best_value = value;
                }
            }
            draw_texture(&best_texture.texture, x * pixel_size, y * pixel_size, WHITE);
        }
    }

    set_default_camera();

    let mut result = render_target.texture.get_texture_data();
    flip_image_vertically(&mut result);
    result
}

fn flip_image_vertically(img: &mut macroquad::texture::Image) {
    let w = img.width as usize;
    let h = img.height as usize;
    let row_bytes = w * 4;

    for y in 0 .. h / 2 {
        let top_start = y * row_bytes;
        let bot_start = (h - 1 - y) * row_bytes;

        for i in 0 .. row_bytes {
            img.bytes.swap(top_start + i, bot_start + i);
        }
    }
}

fn is_likely_image_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext_lower = ext.to_string_lossy().to_lowercase();
        matches!(
            ext_lower.as_str(),
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "ico" | 
            "tiff" | "tif" | "webp" | "pnm" | "tga" | "dds" |
            "farbfeld" | "exr" | "hdr"
        )
    } else {
        false
    }
}

fn load_images_parallel(
    paths: Vec<PathBuf>,
    tx: Sender<LoaderMsg>,
    cancel: CancelToken,
    col_sel: ColSelection,
    pixel_size: Option<u32>
) {
    let total = paths.len();

    let tx_clone = tx.clone();

    paths
        .into_par_iter()
        .try_for_each(|path| {
            if cancel.is_cancelled() {
                return Err(());
            }

            let bytes = match std::fs::read(&path) {
                Ok(bytes) => bytes,
                Err(err) => {
                    let _ = tx_clone.send(LoaderMsg::Error(format!("Error loading image `{}` from storage:\n\n{err}", path.display())));
                    return Err(());
                }
            };

            let img = match image::load_from_memory(&bytes) {
                Ok(img) => img,
                Err(err) => {
                    let _ = tx_clone.send(LoaderMsg::Error(format!("Error making image `{}` from file:\n\n{err}", path.display())));
                    return Err(());
                }
            };

            let (w, h, img) = if let Some(pixel_size) = pixel_size {
                let filter = if pixel_size > img.width() || pixel_size > img.height() {
                    image::imageops::CatmullRom
                } else {
                    image::imageops::Lanczos3
                };
    
                (
                    pixel_size,
                    pixel_size,
                    img.resize_exact(pixel_size, pixel_size, filter)
                )
            } else {
                (img.width(), img.height(), img)
            };

            let img = img.to_rgba8();

            let _ = tx_clone.send(LoaderMsg::Image(RawTexture::new(w as u16, h as u16, img.into_raw(), col_sel)));

            let _ = tx_clone.send(LoaderMsg::Progress {
                loaded: 1,
                total,
                current: path.display().to_string(),
            });

            Ok(())
        })
        .ok();

    drop(tx_clone);

    let _ = tx.send(LoaderMsg::Done);
}

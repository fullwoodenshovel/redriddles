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

#[derive(Debug)]
pub enum LoaderStatus {
    Loading {
        frac: f32,
        current: String
    },
    Done,
    Cancelled,
    GenError(String),
    SaveError(String)
}

#[derive(Clone, Debug)]
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

#[derive(Debug)]
struct AsyncTextureLoader {
    rx: Receiver<LoaderMsg>,
    cancel: CancelToken,
    status: LoaderStatus,
    textures: Vec<Texture>,
    loaded: usize,
    result: Option<Image>
}

pub struct LoaderWrapper {
    loader: Result<AsyncTextureLoader, LoaderStatus>
}

impl LoaderWrapper {
    pub fn with_folder(path: PathBuf, settings: ProcessSettings) -> Self {
        Self {
            loader: AsyncTextureLoader::with_folder(path, settings).map_err(LoaderStatus::GenError)
        }
    }

    pub fn cancel(&mut self) {
        self.loader.as_mut().unwrap().cancel();
    }

    pub fn get_status(&mut self, ctx: &mut AppContextHandler) -> &mut LoaderStatus {
        match &mut self.loader {
            Ok(loader) => loader.get_status(ctx),
            Err(err) => err
        }
    }

    pub fn reset_save_err(&mut self) {
        self.loader.as_mut().unwrap().reset_save_err()
    }

    pub fn export_png(&mut self, path: impl AsRef<std::path::Path>) {
        self.loader.as_mut().unwrap().export_png(path);
    }

    pub fn try_get_img(&self) -> Result<&Image, String> {
        match match self.loader.as_ref() { // match match :)
            Ok(result) => result,
            Err(err) => if let LoaderStatus::GenError(err) = err {
                return Err(err.clone())
            } else {
                panic!("The err variant of LoaderWrapper should always be GenError")
            }
        }.result.as_ref() {
            Some(result) => Ok(result),
            None => Err("First load in a texture file.".to_string())
        }
    }
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
            loaded: 0,
            result: None
        }
    }

    pub fn cancel(&mut self) {
        self.status = LoaderStatus::Cancelled;
        self.cancel.cancel();
    }

    pub fn get_status(&mut self, ctx: &mut AppContextHandler) -> &mut LoaderStatus {
        if !matches!(self.status, LoaderStatus::SaveError(_) | LoaderStatus::GenError(_) | LoaderStatus::Done | LoaderStatus::Cancelled) {
            while let Ok(result) = self.rx.try_recv() {
                match result {
                    LoaderMsg::Progress { loaded, total, current } => {
                        self.loaded += loaded;
                        self.status = LoaderStatus::Loading { frac: self.loaded as f32 / total as f32, current };
                    },
                    LoaderMsg::Image(texture) => self.textures.push(Texture::from_raw(texture)),
                    LoaderMsg::Done => {
                        if self.textures.is_empty() {
                            self.status = LoaderStatus::GenError("Couldn't find any valid image files in that folder.".to_string());
                        } else {
                            self.status = LoaderStatus::Done;
                            self.result = Some(generate_image(std::mem::take(&mut self.textures), ctx));
                            break;
                        }
                    },
                    LoaderMsg::Error(err) => {
                        self.status = LoaderStatus::GenError(err);
                        break;
                    },
                }
            }
        }
        &mut self.status
    }

    pub fn reset_save_err(&mut self) {
        if let LoaderStatus::SaveError(_) = self.status {
            self.status = LoaderStatus::Done;
        } else {
            panic!("Tried to reset save error, but it wasn't in an errored state.");
        }
    }

    pub fn export_png(&mut self, path: impl AsRef<std::path::Path>) {
        if let Err(err) = save_img(self.result.as_ref().unwrap(), path) {
            self.status = LoaderStatus::SaveError(err.to_string())
        }
    }
}

use image::{ImageBuffer, Rgba};
use macroquad::prelude::*;

pub fn save_img(img: &Image, path: impl AsRef<std::path::Path>) -> Result<(), image::ImageError> {
    let width = img.width as u32;
    let height = img.height as u32;
    let bytes = &img.bytes;

    let buffer: ImageBuffer<Rgba<u8>, _> =
        ImageBuffer::from_raw(width, height, bytes.to_vec())
            .ok_or_else(|| image::ImageError::Limits(image::error::LimitError::from_kind(
                image::error::LimitErrorKind::DimensionError,
            )))?;

    buffer.save(path)
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
    } else {
        for pixel in pixels.iter() {
            // store colour difference per texture, stored in a vector of tuples, then generate random nhmber between 0 and 1 and calculate cumulative until exceeding a value
            let x = pixel.pos[0] as f32 - rect.x;
            let y = pixel.pos[1] as f32 - rect.y;
            let col = col_sel.col_from_rgba_arr(pixel.col);
            let mut total = 0.0;
            let a = (settings.place.temperature - 1.0) / settings.place.temperature;

            let mut raw = Vec::new();
            for texture in &textures {
                let cost = col.distance(texture.average);
                let prob = (a * cost).exp();
                total += prob;
                raw.push((texture, prob));
            }

            let scale = 1.0 / total;

            let rand = rand::rand() as f32 / u32::MAX as f32;
            let mut cumulative = 0.0;

            let mut selected_texture = None;

            for (texture, prob) in raw {
                cumulative += prob * scale;
                if cumulative >= rand {
                    selected_texture = Some(texture);
                    break;
                }
            }

            let selected_texture = selected_texture.unwrap_or_else(|| &textures[0]);

            draw_texture(&selected_texture.texture, x * pixel_size, y * pixel_size, WHITE);
        }
    }

    set_default_camera();

    render_target.texture.get_texture_data()
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

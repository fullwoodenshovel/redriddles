use rayon::prelude::*;
use super::*;

use super::Texture;
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
    Done(Texture2D),
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
            load_images_parallel(paths, tx, cancel_clone, process_settings.col_sel);
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

    pub fn get_status(&mut self, pixels: &PixelArray) -> &mut LoaderStatus {
        if !matches!(self.status, LoaderStatus::Error(_) | LoaderStatus::Done(_)) {
            while let Ok(result) = self.rx.try_recv() {
                match result {
                    LoaderMsg::Progress { loaded, total, current } => {
                        self.loaded += loaded;
                        self.status = LoaderStatus::Loading { frac: self.loaded as f32 / total as f32, current };
                    },
                    LoaderMsg::Image(texture) => self.textures.push(Texture::from_raw(texture)),
                    LoaderMsg::Done => { self.status = LoaderStatus::Done(generate_image(std::mem::take(&mut self.textures), pixels.clone())); break },
                    LoaderMsg::Error(err) => { self.status = LoaderStatus::Error(err); break },
                }
            }
        }
        &mut self.status
    }
}

fn generate_image(textures: Vec<Texture>, pixels: PixelArray) -> Texture2D {
    // todo!()
    Texture2D::empty()
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
    col_sel: ColSelection
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

            let img = img.to_rgba8();
            let (w, h) = img.dimensions();

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

use super::*;

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

pub struct AsyncTextureLoader {
    progress: Arc<Mutex<LoaderProgress>>,
    result_receiver: mpsc::Receiver<Result<Vec<Texture>, String>>,
    loader_thread: thread::JoinHandle<()>,
}

pub struct LoaderProgress {
    total_files: usize,
    loaded_files: usize,
    current_file: Option<String>,
    is_done: bool,
    error: Option<String>,
}

impl AsyncTextureLoader {
    pub fn new_recursive(folder: PathBuf, settings: ExportSettings) -> Result<Self, String> {
        let mut files = Vec::new();
        let mut folders = Vec::new();

        folders.push(folder);

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

    pub fn new(paths: Vec<PathBuf>, settings: ExportSettings) -> Self {
        let progress = Arc::new(Mutex::new(LoaderProgress {
            total_files: paths.len(),
            loaded_files: 0,
            current_file: None,
            is_done: false,
            error: None,
        }));
        
        let (result_tx, result_rx) = mpsc::channel();
        let progress_clone = Arc::clone(&progress);
        
        let loader_thread = thread::spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            
            let result = runtime.block_on(load_textures_with_progress(paths, progress_clone, &settings));
            let _ = result_tx.send(result);
        });
        
        Self {
            progress,
            result_receiver: result_rx,
            loader_thread,
        }
    }
    
    pub fn get_progress(&self) -> LoaderProgress {
        self.progress.lock().unwrap().clone()
    }
    
    pub fn try_get_result(&self) -> Option<Result<Vec<Texture>, String>> {
        match self.result_receiver.try_recv() {
            Ok(result) => Some(result),
            Err(mpsc::TryRecvError::Empty) => None,
            Err(mpsc::TryRecvError::Disconnected) => {
                // Thread finished but didn't send result (shouldn't happen)
                Some(Err("Error with generating export.".into()))
            }
        }
    }
    
    pub fn is_done(&self) -> bool {
        self.progress.lock().unwrap().is_done
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

// Async loading function
async fn load_textures_with_progress(
    paths: Vec<PathBuf>,
    progress: Arc<Mutex<LoaderProgress>>,
    settings: &ExportSettings
) -> Result<Vec<Texture>, String> {
    let mut futures = Vec::new();
    
    // Start all loads
    for (i, path) in paths.iter().enumerate() {
        let path_str = path.to_string_lossy().to_string();
        let progress_clone = Arc::clone(&progress);
        
        let future = async move {
            // Update progress before starting
            {
                let mut prog = progress_clone.lock().unwrap();
                prog.current_file = Some(path_str.clone());
            }
            
            // Load texture
            let texture = match load_texture(&path_str).await {
                Ok(t) => t,
                Err(e) => {
                    let mut prog = progress_clone.lock().unwrap();
                    prog.error = Some(format!("Failed to load {}: {}", path_str, e));
                    return Err(format!("Failed to load {}", path_str));
                }
            };
            
            // Process texture (CPU work)
            let processed = Texture::from_texture(texture, settings);
            
            // Update progress after loading
            {
                let mut prog = progress_clone.lock().unwrap();
                prog.loaded_files += 1;
            }
            
            Ok(processed)
        };
        
        futures.push((i, future));
    }
    
    // Collect results in order
    let mut results = Vec::with_capacity(futures.len());
    results.resize_with(futures.len(), || None);
    
    // Await all futures (they're already running)
    for (original_index, future) in futures {
        match future.await {
            Ok(data) => results[original_index] = Some(data),
            Err(e) => return Err(e),
        }
    }
    
    // Mark as done
    {
        let mut prog = progress.lock().unwrap();
        prog.is_done = true;
        prog.current_file = None;
    }
    
    // Convert Vec<Option<T>> to Vec<T>
    Ok(results.into_iter().flatten().collect())
}

// Clone implementation for progress
impl Clone for LoaderProgress {
    fn clone(&self) -> Self {
        Self {
            total_files: self.total_files,
            loaded_files: self.loaded_files,
            current_file: self.current_file.clone(),
            is_done: self.is_done,
            error: self.error.clone(),
        }
    }
}
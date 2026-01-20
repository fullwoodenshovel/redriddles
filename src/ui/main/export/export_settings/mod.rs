use process::LoaderStatus;
mod process;
use std::{ops::{Deref, DerefMut}, path::PathBuf};

use process::AsyncTextureLoader;
use macroquad::prelude::*;
use super::*;

#[derive(Debug)]
pub struct Texture {
    texture: Texture2D,
    average: [f32; 4],
    // noise: f32
}

pub struct RawTexture {
    texture: Vec<u8>,
    width: u16,
    height: u16,
    average: [f32; 4],
    // noise: f32
}

impl RawTexture {
    pub fn new(width: u16, height: u16, pixels: Vec<u8>, col_sel: ColSelection) -> Self {
        Self {
            average: get_average(&pixels, col_sel),
            width,
            height,
            texture: pixels
        }
    }
}

impl Texture {
    pub fn from_raw(texture: RawTexture) -> Self {
        Self {
            texture: Texture2D::from_rgba8(texture.width, texture.height, &texture.texture),
            average: texture.average
        }
    }

    pub fn draw(&mut self, rect: Rect) {
        draw_texture_ex(
            &self.texture,
            rect.x,
            rect.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(rect.w, rect.h)),
                ..Default::default()
            }
        );
    }
}

fn get_average(texture: &[u8], col_sel: ColSelection) -> [f32; 4] {
    let mut sx = 0.0;
    let mut sy = 0.0;
    let mut sz = 0.0;
    let mut sa = 0.0;

    let count = (texture.len() / 4 )as f32;

    let chunks = texture.as_chunks();
    if !chunks.1.is_empty() {
        panic!("When loading texture, the length of the subpixels isnt a multiple of 4");
    }

    let chunks = chunks.0;
    for [r, g, b, a] in chunks.iter() {
        let col = [
            *r as f32 / 255.0,
            *g as f32 / 255.0,
            *b as f32 / 255.0,
            *a as f32 / 255.0,
        ];

        sa += col[3];

        let col = col_sel.col_from_rgba_arr(col);

        let col = col.to_wheel();

        sx += col.0;
        sy += col.1;
        sz += col.2;
    }

    sx /= count;
    sy /= count;
    sz /= count;

    let mut result = col_sel.col_from_wheel(sx, sy, sz).to_rgba();
    result[3] = sa;
    result
}

// enum ColTex {
//     Col([f32; 4]),
//     Tex(Texture)
// }

pub struct ExportSettings {
    pub path: Option<PathBuf>,
    pub process: ProcessSettings
}

impl ExportSettings {
    pub fn new(path: Option<PathBuf>, temperature: f32, col_sel: ColSelection) -> Self {
        Self { path, process: ProcessSettings { temperature, col_sel } }
    }
}

impl Deref for ExportSettings {
    type Target = ProcessSettings;
    fn deref(&self) -> &Self::Target {
        &self.process
    }
}

impl DerefMut for ExportSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.process
    }
}

#[derive(Clone, Copy)]
pub struct ProcessSettings {
    pub temperature: f32,
    pub col_sel: ColSelection,
}

pub struct ExportSettingsNode {
    texture_loader: Option<AsyncTextureLoader>,
}

impl New for ExportSettingsNode {
    fn new(handler: &mut GenHandler) -> Self {
        handler.push_data(ExportSettings::new(None, 0.0, ColSelection::OkLab));
        Self {
            texture_loader: None,
        }
    }
}

impl Node for ExportSettingsNode {
    fn update(&mut self, ctx: &mut AppContextHandler, node: &NodeStore) {
        disabled_ui_button(Rect::new(150.0, 150.0, 300.0, 38.0), "There are no settings yet", DISABLEDCOL);
        let progress_rect = Rect::new(150.0, 180.0, 300.0, 38.0);
        let text_rect = Rect::new(150.0, 210.0, 300.0, 38.0);
        let get_done_rect = |progress| {
            let mut result = progress_rect;
            result.w *= progress;
            result
        };

        match &mut self.texture_loader {
            None => {
                let settings = ctx.store.get::<ExportSettings>();
                match &settings.path {
                    Some(path) => {
                        if sub_ui_button(progress_rect, "Generate result.", DISABLEDCOL, DISABLEDHOVERCOL, node, ctx.user_inputs) {
                            println!("HANDLE THIS ERROR CORRECTLY");
                            let settings = ctx.store.get::<ExportSettings>();
                            self.texture_loader = Some(AsyncTextureLoader::with_folder(path.clone(), settings.process).unwrap());
                        }
                    },
                    None => disabled_ui_button(progress_rect, "Select a folder first.", DISABLEDCOL)
                }
            },
            Some(loader) => {
                match loader.get_status(ctx.store.get::<PixelArray>()) {
                    LoaderStatus::Cancelled => { // todo!() This does not work for some reason.
                        if sub_ui_button(progress_rect, "Result cancelled. Click to enable.", DISABLEDCOL, DISABLEDHOVERCOL, node, ctx.user_inputs) {
                            self.texture_loader = None;
                        }
                    },
                    LoaderStatus::Done(texture) => {
                        if sub_ui_button(progress_rect, "Result generated. Click to save.", ENABLEDCOL, ENABLEDHOVERCOL, node, ctx.user_inputs) &&
                            let Some(out_path) = save_file("Save as")
                        {
                            texture.get_texture_data().export_png(&out_path.display().to_string());
                            self.texture_loader = None;
                        };
                    },
                    LoaderStatus::Loading { frac, current } => {
                        let inner = sub_ui_button(progress_rect, "", DISABLEDCOL, DISABLEDHOVERCOL, node, ctx.user_inputs);
                        let outer = sub_ui_button(get_done_rect(*frac), "Generating. Click to cancel.", ENABLEDCOL, ENABLEDHOVERCOL, node, ctx.user_inputs);
                        cut_text(current, text_rect.w);
                        disabled_ui_button(text_rect, current, WHITE);
                        
                        if outer || inner {
                            self.texture_loader.as_mut().unwrap().cancel();
                        }
                    },
                    LoaderStatus::Error(err) => {
                        multiline_text(text_rect, err);
                        if sub_ui_button(progress_rect, "Error generating.", ENABLEDCOL, ENABLEDHOVERCOL, node, ctx.user_inputs) {
                            self.texture_loader = None;
                        };
                    }
                }
            }
        }
    }

    fn hit_detect(&mut self, pos: Vec2, node: &NodeStore, store: &mut Store) -> Vec<WeakNode> {
        node.hit_detect_children_and_self(pos, store)
    }
}
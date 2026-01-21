use process::LoaderStatus;
mod process;

mod texture;
use texture::RawTexture;

use process::AsyncTextureLoader;
use macroquad::prelude::*;
use super::*;
// todo!() Add exporting just as pixels.
#[derive(Debug)]
pub struct Texture {
    texture: Texture2D,
    average: [f32; 4],
    // noise: f32
}


impl Texture {
    pub fn from_raw(texture: RawTexture) -> Self {
        Self {
            texture: Texture2D::from_rgba8(texture.width, texture.height, &texture.texture),
            average: texture.average
        }
    }
}

pub struct Preview {
    texture_loader: Option<AsyncTextureLoader>,
}

impl New for Preview {
    fn new(handler: &mut GenHandler) -> Self {
        handler.push_data(ExportSettings::new(None, 0.0, ColSelection::OkLab, 16));
        Self {
            texture_loader: None,
        }
    }
}

impl Node for Preview {
    fn update(&mut self, ctx: &mut AppContextHandler, node: &NodeStore) {
        let progress_rect = Rect::new(150.0, 180.0, 300.0, 38.0);
        let text_rect = Rect::new(150.0, 230.0, 300.0, 38.0);
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
                            println!("todo!(); HANDLE THIS ERROR CORRECTLY");
                            let settings = ctx.store.get::<ExportSettings>();
                            self.texture_loader = Some(AsyncTextureLoader::with_folder(path.clone(), settings.process).unwrap()); // <- This unwrap
                        }
                    },
                    None => disabled_ui_button(progress_rect, "Select a folder first.", DISABLEDCOL)
                }
            },
            Some(loader) => {
                match loader.get_status(ctx) {
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
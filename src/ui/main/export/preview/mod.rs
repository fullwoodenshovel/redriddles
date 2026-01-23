use process::LoaderStatus;
mod process;

mod texture;
use texture::RawTexture;

use process::LoaderWrapper;
use macroquad::prelude::*;
use super::*;
// todo!() Add support for changing already existing settings (pixel size, colour space, temperature, transparency filter)
// todo!() Add exporting just as pixels.
// todo!() Add warning if image size is above 1920 x 1080
// BUMP VERSION
// todo!() Add saving colours persistently and colour gradient thing
// todo!() Add workspaces and importing from a file to automatically make the pixels. Make the current drawing an image
// todo!() Add ctrl + z and ctrl + y
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
    texture_loader: Option<LoaderWrapper>,
    texture: Option<Image>
}

impl New for Preview {
    fn new(handler: &mut GenHandler) -> Self {
        handler.push_data(ExportSettings::new(None, 0.01, ColSelection::OkLab, 128, 1.0));
        Self {
            texture_loader: None,
            texture: None
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
                        if sub_ui_button(
                            progress_rect,
                            "Generate result.",
                            DISABLEDCOL,
                            DISABLEDHOVERCOL,
                            node,
                            ctx.user_inputs
                        ) {
                            println!("todo!(); HANDLE THIS ERROR CORRECTLY");
                            let settings = ctx.store.get::<ExportSettings>();
                            self.texture_loader = Some(LoaderWrapper::with_folder(path.clone(), settings.process));
                        }
                    },
                    None => disabled_ui_button(progress_rect, "Select a folder first.", DISABLEDCOL)
                }
            },
            Some(loader) => {
                match loader.get_status() {
                    LoaderStatus::Cancelled => {
                        if sub_ui_button(
                            progress_rect,
                            "Result cancelled. Click to enable.",
                            DISABLEDCOL,
                            DISABLEDHOVERCOL,
                            node,
                            ctx.user_inputs
                        ) {
                            self.texture_loader = None;
                        }
                    },
                    LoaderStatus::Done => {
                        let image = match self.texture.as_ref() {
                            Some(image) => image,
                            None => {
                                self.texture = Some(loader.get_loader_mut().unwrap().generate_image(ctx));
                                self.texture.as_ref().unwrap()
                            }
                        };
                        if sub_ui_button(
                            progress_rect,
                            "Result generated. Click to save.",
                            ENABLEDCOL,
                            ENABLEDHOVERCOL,
                            node,
                            ctx.user_inputs
                        ) &&
                            let Some(out_path) = save_file("Save as")
                        {
                            loader.get_loader_mut().unwrap().export_png(image, &out_path);
                            self.texture_loader = None; // todo!() Make this go into the textures loaded but image not generated state
                        };
                    },
                    LoaderStatus::Loading { frac, current } => {
                        let inner = sub_ui_button(progress_rect, "", DISABLEDCOL, DISABLEDHOVERCOL, node, ctx.user_inputs);
                        disabled_ui_button(get_done_rect(*frac), "Generating. Click to cancel.", ENABLEDCOL);
                        let mut current = current.clone();
                        cut_text(&mut current, text_rect.w);
                        disabled_ui_button(text_rect, &current, WHITE);

                        if inner {
                            loader.get_loader_mut().unwrap().cancel();
                        }
                    },
                    LoaderStatus::GenError(err) => {
                        multiline_text(text_rect, err);
                        if sub_ui_button(
                            progress_rect,
                            "Error generating.",
                            ENABLEDCOL,
                            ENABLEDHOVERCOL,
                            node,
                            ctx.user_inputs
                        ) {
                            self.texture_loader = None;
                        };
                    },
                    LoaderStatus::SaveError(err) => {
                        multiline_text(text_rect, err);
                        if sub_ui_button(
                            progress_rect,
                            "Error saving file.",
                            ENABLEDCOL,
                            ENABLEDHOVERCOL,
                            node,
                            ctx.user_inputs
                        ) {
                            loader.get_loader_mut().unwrap().reset_save_err();
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
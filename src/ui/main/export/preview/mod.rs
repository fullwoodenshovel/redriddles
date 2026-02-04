use process::LoaderStatus;
mod process;

mod texture;
use texture::RawTexture;

use process::{LoaderWrapper, save_img};
use macroquad::prelude::*;

use super::*;
// todo!() Add saving colours persistently and colour gradient thing
// todo!() Add workspaces and importing from a file to automatically make the pixels. Make the current drawing an image
// todo!() Add autosave and workspace settings.
// todo!() Add ctrl + z and ctrl + y
// todo!() Add settings for low memory usage or normal memory usage (load all textures into RAM or only resized ones)

// How workspaces will work:
// On initial run:
//   there will be no workspace selected, and the main name will be in italics with an asterisk
//   when the user triees to delete the window, it will tell them their changes are not saved because they are not in a workspace
//   this tells them to create a new workspace in the workspaces tab or exit without saving
// On subsequent runs:
//   workspaces will be selected automatically from the last workspace, but they can be unselected to return to initial state
//   workspaces are stored as folders with the workspace name inside of the workspaces folder
//   workspaces will store current pixel image, saved colours, current colour, last position, cached dirs
//
// In settings, they can chose to either:
//   save on exit
//   save after some time period
//   do not save automatically (and they have to use ctrl + s) 
//
// data.json will store:
//   shortcuts
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
    texture: Option<Texture2D>
}

impl New for Preview {
    fn new(handler: &mut GenHandler) -> Self {
        handler.push_data(ExportSettings::new(None, 0.0, ColSelection::OkLab, ColSelection::OkLab, 128, 1.0));
        Self {
            texture_loader: None,
            texture: None
        }
    }
}

impl Preview {
    fn update_loader(&mut self, ctx: &mut AppContextHandler, node: &NodeStore) {
        let progress_rect = Rect::new(400.0, 130.0, 300.0, 38.0);
        let text_rect = Rect::new(400.0, 180.0, 300.0, 38.0);
        let get_done_rect = |progress| {
            let mut result = progress_rect;
            result.w *= progress;
            result
        };

        if let Some(texture) = &self.texture {
            let mut target_width = screen_width() - 500.0;
            let mut target_height = screen_height() - 300.0;
            let width = texture.width();
            let height = texture.height();

            let aspect_ratio = width / height;
            let target_aspect = target_width / target_height;

            if aspect_ratio > target_aspect {
                target_height = target_width / aspect_ratio;
            } else {
                target_width = target_height * aspect_ratio;
            }
            
            draw_texture_ex(
                texture,
                400.0,
                180.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(target_width, target_height)),
                    ..Default::default()
                });
        }

        match &mut self.texture_loader {
            None => {
                let settings = ctx.store.get::<ExportSettings>();
                self.texture = None;
                match &settings.path {
                    Some(path) => {
                        if sub_ui_button(
                            progress_rect,
                            "Load textures.",
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
                            "Loading cancelled. Click to enable.",
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
                                self.texture = Some(loader.get_loader_mut().unwrap().generate_image(ctx.store));
                                self.texture.as_ref().unwrap()
                            }
                        };
                        if sub_ui_button(
                            progress_rect,
                            "Loading finished. Click to save.",
                            ENABLEDCOL,
                            ENABLEDHOVERCOL,
                            node,
                            ctx.user_inputs
                        ) &&
                            let Some(out_path) = save_file("Save as")
                        {
                            loader.get_loader_mut().unwrap().export_png(&image.get_texture_data(), &out_path);
                        };
                    },
                    LoaderStatus::Loading { frac, current } => {
                        let inner = sub_ui_button(progress_rect, "", DISABLEDCOL, DISABLEDHOVERCOL, node, ctx.user_inputs);
                        disabled_ui_button(get_done_rect(*frac), "Loading textures. Click to cancel.", ENABLEDCOL);
                        let mut current = current.clone();
                        cut_text(&mut current, text_rect.w);
                        disabled_ui_button(text_rect, &current, WHITE);
    
                        if inner {
                            loader.get_loader_mut().unwrap().cancel();
                        }
                    },
                    LoaderStatus::GenError(err) => {
                        multiline_text(text_rect, err, 18);
                        if sub_ui_button(
                            progress_rect,
                            "Error loading textures.",
                            ENABLEDCOL,
                            ENABLEDHOVERCOL,
                            node,
                            ctx.user_inputs
                        ) {
                            self.texture_loader = None;
                        };
                    },
                    LoaderStatus::SaveError(err) => {
                        multiline_text(text_rect, err, 18);
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
}

impl Node for Preview {
    fn update(&mut self, ctx: &mut AppContextHandler, node: &NodeStore) {        
        let settings = ctx.store.get_mut::<ExportSettings>();
        let place_rect = settings.place.rect;

        if settings.process.changed_this_frame {
            self.texture = None;
            self.texture_loader = None;
            settings.process.changed_this_frame = false;
        }

        let place = &mut settings.place;

        if let Some(texture) = &self.texture {
            let width = texture.width();
            let height = texture.height();
            if width > 1920.0 || height > 1080.0 {
                draw_text(&format!("WARNING: Size of resulting image is large: {width} x {height}"),220.0, 110.0, 18.0, BLACK);
            } else {
                draw_text(&format!("Size of resulting image: {width} x {height}"),220.0, 110.0, 18.0, BLACK);
            }
        }
        
        if let Some(value) = slider(
            ENABLEDCOL,
            DISABLEDCOL,
            Rect::new(50.0, 250.0, 300.0, 18.0),
            &format!("Temperature: {:.2}", place.temperature),
            place.temperature,
            0.0,
            1.0,
            ctx.user_inputs,
            node
        ) {
            place.temperature = value;
        }

        if sub_ui_button(
            Rect::new(50.0, 290.0, 300.0, 26.0), &format!("Distance colour space: {}", settings.place.distance_col),
            DISABLEDCOL,
            DISABLEDHOVERCOL,
            node,
            ctx.user_inputs)
        {
            settings.place.distance_col = settings.place.distance_col.toggle();
        }


        let rect = Rect::new(50.0, 190.0, 300.0, 26.0);
        if let Some(Ok(loader)) = self.texture_loader.as_ref().map(|loader| loader.get_loader()) &&
            loader.is_loaded()
        {
            if sub_ui_button(rect, "Regenerate preview", DISABLEDCOL, DISABLEDHOVERCOL, node, ctx.user_inputs) {
                self.texture = Some(loader.generate_image(ctx.store));
            }
        } else {
            disabled_ui_button(rect, "Succesfully load textures first.", DISABLEDCOL);
        }

        if sub_ui_button(
            Rect::new(50.0, 130.0, 300.0, 38.0),
            "Export image as pixels.",
            ENABLEDCOL,
            ENABLEDHOVERCOL,
            node,
            ctx.user_inputs
        ) && let Some(out_path) = save_file("Save as") {
            let pixels = ctx.store.get::<PixelArray>();
            let rect = place_rect.unwrap_or_else(|| {
                let [WorldPos(x, y), WorldPos(w, h)] = pixels.get_bounds();
                Rect::new(x, y, w - x + 1.0, h - y + 1.0)
            });

            let w = rect.w as u32;
            let h = rect.h as u32;

            let render_target = render_target(w, h);
            render_target.texture.set_filter(FilterMode::Nearest);
            
            set_camera(&Camera2D {
                target: vec2(rect.w, rect.h) / 2.0,
                zoom: 2.0 / vec2(rect.w, rect.h),
                render_target: Some(render_target.clone()),
                ..Default::default()
            });
            
            clear_background(BLANK);
            for pixel in pixels.iter() {
                draw_rectangle(pixel.pos[0] as f32 - rect.x, pixel.pos[1] as f32 - rect.y, 1.0, 1.0, arr_to_macroquad(pixel.col));
            }

            set_default_camera();

            // todo!() fix this v
            // This let is not the best, but with the architecture im using to display errors theres not currently a good solution.
            let _ = save_img(&render_target.texture.get_texture_data(), out_path);
        }

        self.update_loader(ctx, node);
    }

    fn hit_detect(&mut self, pos: Vec2, node: &NodeStore, store: &mut Store) -> Vec<WeakNode> {
        node.hit_detect_children_and_self(pos, store)
    }
}
use std::path::PathBuf;
use opener::open;
use std::fs;

use super::*;

pub struct SelectFolder {
    selected_path: Option<PathBuf>
}

impl New for SelectFolder {
    fn new(_handler: &mut GenHandler) -> Self {
        Self {
            selected_path: None
        }
    }
}

impl Node for SelectFolder {
    fn update(&mut self, ctx: &mut AppContextHandler, node: &NodeStore) {
        let rect = Rect::new(28.0, 100.0, 150.0, 38.0);
        if sub_ui_button(rect, "Select new folder", ENABLEDCOL, ENABLEDHOVERCOL, node, ctx) && let Some(folder) = pick_folder() {
            ctx.save_data.cached_dirs.retain(|d| *d != folder);
            ctx.save_data.cached_dirs.push_front(folder);
            if ctx.save_data.cached_dirs.len() > 10 {
                ctx.save_data.cached_dirs.pop_back();
            }
            self.selected_path = Some(ctx.save_data.cached_dirs.front().unwrap().clone());
        }

        let rect = Rect::new(28.0, 150.0, screen_width() - 200.0, 28.0);
        
        for (index, path) in ctx.save_data.cached_dirs.clone().iter().enumerate() {
            let mut display: String = path.to_str().unwrap_or("This folder path cannot be displayed as it is not valid UTF-8").to_string();
            if measure_text(&display, None, 18, 1.0).width > rect.w - 16.0 {
                display = format!("...{display}");
                while display.len() > 3 && measure_text(&display, None, 18, 1.0).width > rect.w - 16.0 {
                    display.remove(3);
                }
            }
            if sub_ui_button(
                Rect::new(28.0, 150.0 + 38.0 * index as f32, rect.w, 28.0),
                &display,
                if let Some(sel_path) = &self.selected_path && path == sel_path {ENABLEDCOL} else {DISABLEDCOL},
                if let Some(sel_path) = &self.selected_path && path == sel_path {ENABLEDHOVERCOL} else {DISABLEDHOVERCOL},
                node,
                ctx
            ) {
                ctx.save_data.cached_dirs.remove(index);
                ctx.save_data.cached_dirs.push_front(path.clone());
                self.selected_path = Some(path.clone())
            }
            if sub_ui_button(
                Rect::new(rect.w + 50.0, 150.0 + 38.0 * index as f32, 100.0, 28.0),
                "Open folder",
                DISABLEDCOL,
                DISABLEDHOVERCOL,
                node,
                ctx
            ) {
                let exists = fs::exists(path);
                if let Ok(false) = exists {
                    eprintln!("Folder `{path:?}` does not exist.")
                } else if let Err(err) = exists {
                    eprintln!("Error opening folder `{path:?}`:\n{err}")
                }

                if let Err(err) = open(path) {
                    eprintln!("Error opening folder `{path:?}`:\n{err}");
                }
            }
        }
    }

    fn hit_detect(&mut self, pos: Vec2, node: &NodeStore, store: &mut Store) -> Vec<WeakNode> {
        node.hit_detect_children_and_self(pos, store)
    }
}
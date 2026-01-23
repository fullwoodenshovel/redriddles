use std::path::PathBuf;
use opener::open;
use std::fs;

use super::*;

pub struct SelectFolder {
}

impl New for SelectFolder {
    fn new(_handler: &mut GenHandler) -> Self {
        Self {
        }
    }
}

impl Node for SelectFolder {
    fn update(&mut self, ctx: &mut AppContextHandler, node: &NodeStore) {
        let rect = Rect::new(28.0, 100.0, 150.0, 38.0);
        let selected_path: &mut Option<PathBuf> = &mut ctx.store.get_mut::<ExportSettings>().path;
        if sub_ui_button(rect, "Select new folder", ENABLEDCOL, ENABLEDHOVERCOL, node, ctx.user_inputs) &&
            let Some(folder) = pick_folder("Select a folder")
        {
            ctx.save_data.cached_dirs.retain(|d| *d != folder);
            ctx.save_data.cached_dirs.push_front(folder);
            if ctx.save_data.cached_dirs.len() > 10 {
                ctx.save_data.cached_dirs.pop_back();
            }
            *selected_path = Some(ctx.save_data.cached_dirs.front().unwrap().clone());
        }

        let rect = Rect::new(28.0, 150.0, screen_width() - 200.0, 28.0);
        
        for (index, path) in ctx.save_data.cached_dirs.clone().iter().enumerate() {
            let mut display: String = path.to_str().unwrap_or("This folder path cannot be displayed as it is not valid UTF-8").to_string();
            cut_text(&mut display, rect.w);
            if sub_ui_button(
                Rect::new(28.0, 150.0 + 38.0 * index as f32, rect.w, 28.0),
                &display,
                if let Some(sel_path) = selected_path && path == sel_path {ENABLEDCOL} else {DISABLEDCOL},
                if let Some(sel_path) = selected_path && path == sel_path {ENABLEDHOVERCOL} else {DISABLEDHOVERCOL},
                node,
                ctx.user_inputs
            ) {
                ctx.save_data.cached_dirs.remove(index);
                ctx.save_data.cached_dirs.push_front(path.clone());
                *selected_path = Some(path.clone())
            }
            if sub_ui_button(
                Rect::new(rect.w + 50.0, 150.0 + 38.0 * index as f32, 100.0, 28.0),
                "Open folder",
                DISABLEDCOL,
                DISABLEDHOVERCOL,
                node,
                ctx.user_inputs
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
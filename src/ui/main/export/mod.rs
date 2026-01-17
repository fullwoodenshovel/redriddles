use rfd::FileDialog;

use super::*;
mod select_folder;
use select_folder::SelectFolder;
mod export_settings;
use export_settings::{ExportSettings, ExportSettingsNode};
mod preview;
use preview::Preview;

fn pick_folder() -> Option<std::path::PathBuf> {
    FileDialog::new()
        .set_title("Select a folder")
        .pick_folder()
}

#[derive(Default)]
pub struct Export {
}

const TOPBAR: usize = 3;
impl New for Export { // 0 is select folder, 1 is export settings, 2 is preview, 3 is topbar,
    fn new(handler: &mut GenHandler) -> Self {
        status::push_nocheck::<1>(handler);
        
        handler.push_child::<SelectFolder>();
        handler.push_child::<ExportSettingsNode>();
        handler.push_child::<Preview>();

        handler.push_child_io::<Topbar<1>>((
            156.0,
            "Export",
            Box::new([
                "Select folder",
                "Export settings",
                "Preview",
            ])
        ));

        Self {
        }
    }
}

impl Node for Export {
    fn update(&mut self, ctx: &mut AppContextHandler, node: &NodeStore) {
        clear_background(WHITE);

        let children = node.get_children();
        let status = status::get_or_default::<1>(ctx.store);
        children[status as usize].update(ctx);
        children[TOPBAR].update(ctx);
    }
    
    fn hit_detect(&mut self, pos: Vec2, node: &NodeStore, store: &mut Store) -> Vec<WeakNode> {
        let children = node.get_children();
        let mut result = children[TOPBAR].hit_detect(pos, store);
        if result.is_empty() {
            result = node.get_children()[status::get_or_default::<1>(store) as usize].hit_detect(pos, store);
        }
        result.push(node.get_weak());
        result
    }
}
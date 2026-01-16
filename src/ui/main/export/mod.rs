use rfd::FileDialog;

use super::*;
mod export_settings;
mod preview;
mod select_folder;
use select_folder::SelectFolder;

fn pick_folder() -> Option<std::path::PathBuf> {
    FileDialog::new()
        .set_title("Select a folder")
        .pick_folder()
}

#[derive(Default)]
pub struct Export {
}

impl New for Export { // 0 is select folder, 1 is topbar,
    fn new(handler: &mut GenHandler) -> Self {
        status::push_nocheck::<1>(handler);
        
        handler.push_child::<SelectFolder>();

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
        children[1].update(ctx);
    }
    
    fn hit_detect(&mut self, pos: Vec2, node: &NodeStore, store: &mut Store) -> Vec<WeakNode> {
        let children = node.get_children();
        let mut result = children[1].hit_detect(pos, store);
        if result.is_empty() {
            result = node.get_children()[status::get_or_default::<1>(store) as usize].hit_detect(pos, store);
        }
        result.push(node.get_weak());
        result
    }
}
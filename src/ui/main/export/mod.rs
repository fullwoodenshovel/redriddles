use rfd::FileDialog;

use super::*;
mod sidebar;
use sidebar::Sidebar;

fn pick_folder() -> Option<std::path::PathBuf> {
    FileDialog::new()
        .set_title("Select a folder")
        .pick_folder()
}

#[derive(Default)]
pub struct Export {
}

impl New for Export {
    fn new(handler: &mut GenHandler) -> Self {
        handler.push_child::<Sidebar>();
        Self {
        }
    }
}

impl Node for Export {
    fn update(&mut self, _ctx: &mut AppContextHandler, _node: &NodeStore) {
        println!("NOW");
        println!("FOLDER: {:?}", pick_folder());
    }
    
    fn hit_detect(&mut self, pos: Vec2, node: &NodeStore, store: &mut Store) -> Vec<WeakNode> {
        node.hit_detect_children_and_self(pos, store)
    }
}
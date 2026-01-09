use colour::ColSelection;

use super::*;
mod sidebar;
mod draw_section;
use sidebar::Sidebar;
pub use sidebar::DrawState;
use draw_section::DrawSection;

pub struct Draw;

impl New for Draw {
    fn new(handler: &mut GenHandler) -> Self { // 0 is Sidebar, 1 is DrawSection
        handler.push_child::<Sidebar>();
        handler.push_child::<DrawSection>();
        
        Self
    }
}

impl Node for Draw {
    fn update(&mut self, ctx: &mut AppContextHandler, node: &NodeStore) {
        clear_background(WHITE);
        for child in node.get_children().iter().rev() {
            child.update(ctx);
        }
    }

    fn hit_detect(&mut self, pos: Vec2, node: &NodeStore, store: &mut Store) -> Vec<WeakNode> {
        node.hit_detect_children_and_self(pos, store)
    }
}
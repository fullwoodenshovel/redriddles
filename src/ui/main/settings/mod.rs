use super::*;

mod shortcuts;
use shortcuts::Shortcuts;

pub struct Settings;

impl New for Settings {
    fn new(handler: &mut GenHandler) -> Self {
        handler.push_child::<Shortcuts>();
        Self
    }
}

impl Node for Settings {
    fn update(&mut self, _ctx: &mut AppContextHandler, _node: &NodeStore) {
    }
    
    fn hit_detect(&mut self, pos: Vec2, node: &NodeStore, store: &mut Store) -> Vec<WeakNode> {
        node.hit_detect_children_and_self(pos, store)
    }
}
use super::*;

pub struct Shortcuts;

impl New for Shortcuts {
    fn new(_handler: &mut GenHandler) -> Self {
        Self
    }
}

impl Node for Shortcuts {
    fn update(&mut self, _ctx: &mut AppContextHandler, _node: &NodeStore) {
    }
    
    fn hit_detect(&mut self, pos: Vec2, node: &NodeStore, store: &mut Store) -> Vec<WeakNode> {
        node.hit_detect_children_and_self(pos, store)
    }
}
use super::*;

pub struct Preview {

}

impl New for Preview {
    fn new(_handler: &mut GenHandler) -> Self {
        Self {

        }
    }
}

impl Node for Preview {
    fn update(&mut self, ctx: &mut AppContextHandler, node: &NodeStore) {
        disabled_ui_button(Rect::new(150.0, 150.0, 300.0, 38.0), "This does not yet exist", DISABLEDCOL)
    }

    fn hit_detect(&mut self, pos: Vec2, node: &NodeStore, store: &mut Store) -> Vec<WeakNode> {
        node.hit_detect_children_and_self(pos, store)
    }
}
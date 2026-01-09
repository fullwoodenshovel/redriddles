use rfd::FileDialog;

use super::*;

pub struct Sidebar {

}

impl New for Sidebar {
    fn new(_handler: &mut GenHandler) -> Self {
        Self {
        }
    }
}

impl Node for Sidebar {
    fn update(&mut self, ctx: &mut AppContextHandler, node: &NodeStore) {
        // macroquad::texture::load_image("/home/fullw/Downloads/e4ee9220524d977c9e9f25a064139512.png").await
    }

    fn hit_detect(&mut self, pos: Vec2, node: &NodeStore, store: &mut Store) -> Vec<WeakNode> {
        node.hit_detect_children_and_self(pos, store)
    }
}
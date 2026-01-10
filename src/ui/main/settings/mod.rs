use super::*;

mod shortcuts;
use shortcuts::Shortcuts;

pub struct Settings;

impl New for Settings {
    fn new(handler: &mut GenHandler) -> Self {
        handler.push_child::<Shortcuts>();

        handler.push_data(Status::<1>(0));
        handler.push_child_io::<Topbar<1>>((
            156.0,
            "Settings",
            Box::new([
                "Something",
                "Completely",
                "Different",
            ])
        ));

        Self
    }
}

impl Node for Settings {
    fn update(&mut self, ctx: &mut AppContextHandler, node: &NodeStore) {
        clear_background(WHITE);
        node.update_children(ctx);
    }
    
    fn hit_detect(&mut self, pos: Vec2, node: &NodeStore, store: &mut Store) -> Vec<WeakNode> {
        node.hit_detect_children_and_self(pos, store)
    }
}
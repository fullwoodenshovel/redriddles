use super::*;

mod shortcuts;
use shortcuts::Shortcuts;

pub struct Settings;

impl New for Settings {
    // 0 is shortcuts, 1 is topbar
    // THINGS ARE DEPENDENT ON THIS. Change with care.
    fn new(handler: &mut GenHandler) -> Self { 
        handler.push_child::<Shortcuts>();

        status::push::<1>(handler);
        handler.push_child_io::<Topbar<1>>((
            156.0,
            "Settings",
            Box::new([
                "Shortcuts"
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
        let children = node.get_children();
        let mut result = children[1].hit_detect(pos, store);
        if result.is_empty() {
            result = node.get_children()[status::get_or_default::<1>(store) as usize].hit_detect(pos, store);
        }
        result.push(node.get_weak());
        result
    }
}
use super::*;
mod topbar;
mod draw;
mod settings;
mod export;
use topbar::Topbar;
use draw::Draw;
use settings::Settings;
use export::Export;
pub use draw::DrawState;

pub struct Main {
}

#[tuple_deref]
pub struct Status<const INDEX: u8>(u8);

impl New for Main {
    fn new(handler: &mut GenHandler) -> Self { // 0 is topbar, 1 is draw, 2 is settings, 3 is export
        handler.push_data(Status::<0>(1));
        handler.push_child_io::<Topbar<0>>(
            Box::new([
                ("Draw", Rect::new(160.0, 5.0, 75.0, 28.0), 1),
                ("Settings", Rect::new(240.0, 5.0, 75.0, 28.0), 2),
                ("Export", Rect::new(320.0, 5.0, 75.0, 28.0), 3),
            ])
        );
        handler.push_child::<Draw>();
        handler.push_child::<Settings>();
        handler.push_child::<Export>();
        Self {}
    }
}

impl Node for Main {
    fn update(&mut self, ctx: &mut AppContextHandler, node: &NodeStore) {
        let children = node.get_children();
        let status = ctx.store.value::<Status<0>>();
        children[status as usize].update(ctx);
        if status == 3 {
            ctx.store.set::<Status<0>>(1);
        } // TEMPORARY. THIS DISABLES EXPORT TAB, IT ONLY GETS ENABLED FOR ONE FRAME
        children[0].update(ctx);
    }

    fn hit_detect(&mut self, pos: Vec2, node: &NodeStore, store: &mut Store) -> Vec<WeakNode> {
        let children = node.get_children();
        let mut result = children[0].hit_detect(pos, store);
        if result.is_empty() {
            result = node.get_children()[store.value::<Status<0>>() as usize].hit_detect(pos, store);
        }
        result.push(node.get_weak());
        result
    }
}
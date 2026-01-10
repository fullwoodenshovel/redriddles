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
    fn new(handler: &mut GenHandler) -> Self { // 0 is draw, 1 is settings, 2 is export, 3 is topbar
                                               // THINGS ARE DEPENDENT ON THIS. Change with care.
        handler.push_child::<Draw>();
        handler.push_child::<Settings>();
        handler.push_child::<Export>();

        handler.push_data(Status::<0>(0));
        handler.push_child_io::<Topbar<0>>((
            156.0,
            "Pixel Editor",
            Box::new([
                "Draw",
                "Settings",
                "Export",
            ])
        ));

        Self {}
    }
}

impl Node for Main {
    fn update(&mut self, ctx: &mut AppContextHandler, node: &NodeStore) {
        let children = node.get_children();
        let status = ctx.store.value::<Status<0>>();
        children[status as usize].update(ctx);
        if status == 2 {
            ctx.store.set::<Status<0>>(0);
        } // TEMPORARY. THIS DISABLES EXPORT TAB, IT ONLY GETS ENABLED FOR ONE FRAME
        children[3].update(ctx);
    }

    fn hit_detect(&mut self, pos: Vec2, node: &NodeStore, store: &mut Store) -> Vec<WeakNode> {
        let children = node.get_children();
        let mut result = children[3].hit_detect(pos, store);
        if result.is_empty() {
            result = node.get_children()[store.value::<Status<0>>() as usize].hit_detect(pos, store);
        }
        result.push(node.get_weak());
        result
    }
}
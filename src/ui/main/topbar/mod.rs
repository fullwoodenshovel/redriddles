use super::*;
mod change_status;
use change_status::ChangeStatus;

pub struct Topbar<const INDEX: u8> {
    size: Vec2
}

#[tuple_deref]
pub struct HoverPossible(bool);

impl<const INDEX: u8> NewNoOut for Topbar<INDEX> {
    type InType = Box<[(&'static str, Rect, u8)]>;
    fn new(data: Self::InType, handler: &mut GenHandler) -> Self {
        handler.push_data(HoverPossible(false));
        for (name, rect, index) in data {
            handler.push_child_io::<ChangeStatus<INDEX>>((name.to_string(), rect, index));
        }

        Self {
            size: Vec2::new(screen_width(), 40.0)
        }
    }
}

impl<const INDEX: u8> Node for Topbar<INDEX> {
    fn update(&mut self, ctx: &mut AppContextHandler, node: &NodeStore) {
        self.size.x = screen_width();
        let starty = INDEX as f32 * 40.0;
        draw_rectangle(0.0, starty, self.size.x, self.size.y, DARKGRAY);
        draw_text("Pixel Editor.", 10.0, starty + 26.0, 22.0, WHITE);
        **ctx.store.get_mut::<HoverPossible>() = ctx.user_inputs.hoverhold_test(node);
        node.update_children(ctx);
    }

    fn hit_detect(&mut self, pos: Vec2, node: &NodeStore, _store: &mut Store) -> Vec<WeakNode> {
        if self.bounding_box().contains(pos) {
            vec![node.get_weak()]
        } else {
            vec![]
        }
    }
}

impl<const INDEX: u8> Topbar<INDEX> {
    fn bounding_box(&self) -> Rect {
        Rect::new(0.0, INDEX as f32 * 40.0, self.size.x, self.size.y)
    }
}
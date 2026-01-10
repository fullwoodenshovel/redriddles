use super::*;
mod change_status;
use change_status::ChangeStatus;

pub struct Topbar<const INDEX: u8> {
    size: Vec2,
    label: &'static str
}

#[tuple_deref]
pub struct Status<const INDEX: u8>(pub u8);

impl<const INDEX: u8> NewNoOut for Topbar<INDEX> {
    type InType = (f32, &'static str, Box<[&'static str]>);
    fn new((mut x_offset, label, names): Self::InType, handler: &mut GenHandler) -> Self {
        for (index, name) in names.iter().enumerate() {
            x_offset += 6.0 + handler.push_child_io::<ChangeStatus<INDEX>>((name.to_string(), x_offset, index as u8));
        }

        Self {
            size: Vec2::new(screen_width(), 40.0),
            label
        }
    }
}

impl<const INDEX: u8> Node for Topbar<INDEX> {
    fn update(&mut self, ctx: &mut AppContextHandler, node: &NodeStore) {
        self.size.x = screen_width();
        let starty = INDEX as f32 * 42.0;
        if INDEX != 0 {
            draw_rectangle(0.0, starty - 2.0, self.size.x, 2.0, BLACK);
        }
        draw_rectangle(0.0, starty, self.size.x, self.size.y, DARKGRAY);

        draw_text(self.label, 10.0, starty + 26.0, 22.0, WHITE);
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
        Rect::new(0.0, INDEX as f32 * 42.0, self.size.x, self.size.y)
    }
}
use super::*;

pub struct Topbar {
    start_pos: Vec2,
    size: Vec2,
}

impl NewNoOut for Topbar {
    type InType = ();
    fn new(_: Self::InType, _handler: &mut GenHandler) -> Self {
        Self {
            start_pos: Vec2::new(0.0, 0.0),
            size: Vec2::new(screen_width(), 40.0)
        }
    }
}

impl Node for Topbar {
    fn update(&mut self, _user_inputs: &UserInputs, _node: &NodeStore) {
        self.size.x = screen_width();
        draw_rectangle(self.start_pos.x, self.start_pos.y, self.size.x, self.size.y, DARKGRAY);
        draw_text("Pixel Editor (Zoom & Pan)", 10.0, 26.0, 22.0, WHITE);
    }

    fn hit_detect(&mut self, pos: Vec2, node: &NodeStore) -> Vec<WeakNode> {
        if self.bounding_box().contains(pos) {
            vec![node.get_weak()]
        } else {
            vec![]
        }
    }
}

impl Topbar {
    fn bounding_box(&self) -> Rect {
        Rect::new(self.start_pos.x, self.start_pos.y, self.size.x, self.size.y)
    }
}
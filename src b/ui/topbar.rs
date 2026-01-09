use super::*;

pub struct Topbar {
    start_pos: Vec2,
    size: Vec2,
    node: WeakNode
}

impl New for Topbar {
    type InType = ();
    fn new(_: Self::InType, node: WeakNode) -> Self {
        Self {
            start_pos: Vec2::new(0.0, 0.0),
            size: Vec2::new(screen_width(), 40.0),
            node
        }
    }
}

impl Node for Topbar {
    fn update(&mut self, _user_inputs: &UserInputs) {
        self.size.x = screen_width();
        draw_rectangle(self.start_pos.x, self.start_pos.y, self.size.x, self.size.y, DARKGRAY);
        draw_text("Pixel Editor (Zoom & Pan)", 10.0, 26.0, 22.0, WHITE);
    }

    fn hit_detect(&self, pos: Vec2) -> Vec<WeakNode> {
        if self.bounding_box().contains(pos) {
            vec![self.node.clone()]
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
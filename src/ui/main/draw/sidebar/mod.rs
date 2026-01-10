use super::*;

mod hex_input;
mod save;
mod eraser;
mod save_grid;
mod draw_state_button;

use hex_input::HexInput;
use save::Save;
use eraser::Eraser;
use save_grid::SaveGrid;
use draw_state_button::DrawStateButton;
pub use draw_state_button::DrawState;


use colour_picker::{PickerNode, Picker, ColPicker};
use helpers::*;

pub struct Sidebar {
    start_pos: Vec2,
    size: Vec2,
}

impl New for Sidebar {
    fn new(handler: &mut GenHandler) -> Self { // 0 is picker, 1 is hex input, 2 is save, 3 is save grid, 4 is eraser, 5 is screen picker, 6 is line, 7 is fill
        handler.push_child::<PickerNode>();
        handler.push_child::<HexInput>();
        handler.push_child::<Save>();
        handler.push_child::<SaveGrid>();
        handler.push_child::<Eraser>();

        handler.push_data(DrawState::Draw);
        handler.push_child_io::<DrawStateButton>(("Pick Colour", Rect::new(10.0, 448.0, 133.0, 28.0), DrawState::Picker));
        handler.push_child_io::<DrawStateButton>(("Line", Rect::new(10.0, 516.0, 133.0, 28.0), DrawState::Line));
        handler.push_child_io::<DrawStateButton>(("Fill", Rect::new(10.0, 550.0, 133.0, 28.0), DrawState::Fill));

        Self {
            start_pos: Vec2::new(0.0, 40.0),
            size: Vec2::new(150.0, screen_height() - 40.0),
        }
        
    }
}

impl Node for Sidebar {
    fn update(&mut self, ctx: &mut AppContextHandler, node: &NodeStore) {
        self.size.y = screen_height() - 40.0;

        draw_rectangle(self.start_pos.x, self.start_pos.y, self.size.x, self.size.y, GRAY);

        node.update_children(ctx);
    }

    fn hit_detect(&mut self, pos: Vec2, node: &NodeStore, store: &mut Store) -> Vec<WeakNode> {
        if Rect::new(self.start_pos.x, self.start_pos.y, self.size.x, self.size.y).contains(pos) {
            node.hit_detect_children_and_self(pos, store)
        } else {
            vec![]
        }
    }
}
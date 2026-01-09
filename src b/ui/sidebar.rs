use super::*;
use hex_input::HexInput;
use colour_picker::{PickerNode, Picker, picker::Circular};
use super::super::helpers::*;

pub struct Sidebar {
    start_pos: Vec2,
    size: Vec2,
    saved_cols: Vec<[Option<[f32; 4]>; 4]>,
    hex_input: Option<StrongRef<HexInput>>,
    save_rect: Rect,
    screen_picker_rect: Rect,
    node: WeakNode // 0 is hex input, 1 is picker
}

impl New for Sidebar {
    type InType = Option<Picker>;
    fn new(picker: Self::InType, node: WeakNode) -> Self {
        let upgrade = node.upgrade().unwrap();
        upgrade.borrow_mut().push_child::<HexInput>((
            Rect::new(10.0, 414.0, 100.0, 28.0),
            LIGHTGRAY,
            Color { r: 0.65, g: 0.65, b: 0.65, a: 1.0 },
            Color { r: 0.65, g: 0.8, b: 0.65, a: 1.0 }
        ));
        upgrade.borrow_mut().push_child::<PickerNode>(
            if let Some(picker) = picker {
                picker
            } else {
                Picker::Circular(Circular::new(50.0, 10.0, [10.0, 270.0], 16.0, ColSelection::Hsva))
            }
        );

        Self {
            start_pos: Vec2::new(0.0, 40.0),
            size: Vec2::new(150.0, screen_height() - 40.0),
            saved_cols: vec![[None; 4]; 6],
            hex_input: None,
            save_rect: Rect::new(10.0, 380.0, 100.0, 28.0),
            screen_picker_rect: Rect::new(10.0, 448.0, 100.0, 28.0),
            node
        }
    }
}

impl Node for Sidebar {
    fn update(&mut self, user_inputs: &UserInputs) {
        self.size.y = screen_height() - 40.0;
        let node = self.node.upgrade().unwrap();
        let hoverhold = user_inputs.hoverhold_test(&node);
        let binding = node.borrow();
        let mut binding = binding.children[1].borrow_mut();
        let picker = binding.get_self_mut::<PickerNode>();

        draw_rectangle(self.start_pos.x, self.start_pos.y, self.size.x, self.size.y, GRAY);

        for (y, cols) in self.saved_cols.iter_mut().enumerate() {
            for (x, col) in cols.iter_mut().enumerate() {
                let x = 33.0 * x as f32 + 10.0;
                let y = 33.0 * y as f32 + 60.0;

                if col_button(
                    Rect::new(x, y, 28.0, 28.0),
                    if hoverhold {Some(user_inputs.mouse)} else {None},
                    user_inputs.left_let_go, // If it aint working, this is probably the line of code responsible. It also might be the stuff inside this if block.
                    if hoverhold { Color { r: 0.65, g: 0.8, b: 0.65, a: 1.0 } } else { LIGHTGRAY },
                    if hoverhold { Color { r: 0.60, g: 0.75, b: 0.60, a: 1.0 } } else { Color { r: 0.65, g: 0.65, b: 0.65, a: 1.0 } }
                ) {
                    if hoverhold {
                        *col = picker.get_col_rgba();
                    } else {
                        picker.set_col(*col);
                    }
                }

                if let Some(col) = col {
                    draw_rectangle(x + 4.0, y + 4.0, 20.0, 20.0, arr_to_macroquad(*col));
                }
            }
        }
    }

    fn hit_detect(&self, pos: Vec2) -> Vec<WeakNode> {
        if Rect::new(self.start_pos.x, self.start_pos.y, self.size.x, self.size.y).contains(pos) {
            recur_hit_detect(&self.node, pos)
        } else {
            vec![]
        }
    }
}
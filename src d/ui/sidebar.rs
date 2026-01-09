use std::{cell::RefCell, rc::Rc};

use super::*;
use hex_input::HexInput;
use colour_picker::{PickerNode, Picker, ColPicker};
use super::super::helpers::*;

pub struct Sidebar {
    start_pos: Vec2,
    size: Vec2,
    saved_cols: Vec<[Option<[f32; 4]>; 4]>,
    save_rect: Rect,
    screen_picker_rect: Rect,
    picker: Rc<RefCell<Picker>>,
    hover_possible: Rc<RefCell<bool>>
}

impl New for Sidebar {
    type InType = ();
    type OutType = Rc<RefCell<Picker>>;
    fn new(_: Self::InType, handler: &mut GenHandler) -> (Self::OutType, Self) { // 0 is picker, 1 is hex input
        let picker = handler.push_child::<PickerNode>(());
        let hover_possible = Rc::new(RefCell::new(false));
        handler.push_child::<HexInput>((
            Rect::new(10.0, 414.0, 100.0, 28.0),
            LIGHTGRAY,
            Color { r: 0.65, g: 0.65, b: 0.65, a: 1.0 },
            Color { r: 0.65, g: 0.8, b: 0.65, a: 1.0 },
            picker.clone(),
            hover_possible.clone()
        ));

        (
            picker.clone(),
            Self {
                start_pos: Vec2::new(0.0, 40.0),
                size: Vec2::new(150.0, screen_height() - 40.0),
                saved_cols: vec![[None; 4]; 6],
                save_rect: Rect::new(10.0, 380.0, 100.0, 28.0),
                screen_picker_rect: Rect::new(10.0, 448.0, 100.0, 28.0),
                picker,
                hover_possible
            }
        )
    }
}

impl Node for Sidebar {
    fn update(&mut self, user_inputs: &UserInputs, node: &NodeStore) {
        self.size.y = screen_height() - 40.0;
        let hoverhold = user_inputs.hoverhold_test(node);
        *self.hover_possible.borrow_mut() = hoverhold;

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
                        *col = self.picker.borrow_mut().get_col_rgba();
                    } else {
                        self.picker.borrow_mut().set_col(*col);
                    }
                }

                if let Some(col) = col {
                    draw_rectangle(x + 4.0, y + 4.0, 20.0, 20.0, arr_to_macroquad(*col));
                }
            }
        }

        node.update_children(user_inputs);
    }

    fn hit_detect(&mut self, pos: Vec2, node: &NodeStore) -> Vec<WeakNode> {
        if Rect::new(self.start_pos.x, self.start_pos.y, self.size.x, self.size.y).contains(pos) {
            node.hit_detect_children_and_self(pos)
        } else {
            vec![]
        }
    }
}



// // ---------------- SAVE COLOUR ----------------
// let mut flag = false;
// if let Some(col) = picker.get_col_rgba() {
//     flag = true;
    
//     helpers::ui_button(
//         save_rect,
//         "Save colour",
//         if Focus::Sidebar == focus {Some(mouse)} else {None},
//         let_go,
//         if last_touch_focus == LastTouchFocus::SaveCol { Color { r: 0.65, g: 0.8, b: 0.65, a: 1.0 } } else { LIGHTGRAY },
//         if last_touch_focus == LastTouchFocus::SaveCol { Color { r: 0.60, g: 0.75, b: 0.60, a: 1.0 } } else { Color { r: 0.65, g: 0.65, b: 0.65, a: 1.0 } }
//     );

//     draw_rectangle(115.0, 380.0, 28.0, 28.0, LIGHTGRAY);
//     draw_rectangle(120.0, 385.0, 18.0, 18.0, arr_to_macroquad(col));
// }

// if flag && let Some(col) = hex_input.ui(mouse, Focus::Sidebar == focus, LastTouchFocus::TypeCol == last_touch_focus, let_go) {
//     picker.set_col(Some(col));
// }

// // ---------------- PICK COLOUR ----------------
// helpers::ui_button(
//     screen_picker_rect,
//     "Pick colour",
//     if Focus::Sidebar == focus {Some(mouse)} else {None},
//     let_go,
//     if last_touch_focus == LastTouchFocus::Picker { Color { r: 0.65, g: 0.8, b: 0.65, a: 1.0 } } else { LIGHTGRAY },
//     if last_touch_focus == LastTouchFocus::Picker { Color { r: 0.60, g: 0.75, b: 0.60, a: 1.0 } } else { Color { r: 0.65, g: 0.65, b: 0.65, a: 1.0 } }
// );

// if last_touch_focus == LastTouchFocus::None && prev_touch_focus == LastTouchFocus::Picker && focus == Focus::Draw {
//     let world = transform.screen_to_world(&ScreenPos(mouse.x, mouse.y));
//     let world = world.as_i16();
//     if let Some(pos) = world && let Some(pixel) = pixels.get(pos) {
//         picker.set_col(Some(pixel.col));
//     }
// }

// // ---------------- Rubber ----------------
// if helpers::ui_button(
//     Rect::new(10.0, 482.0, 100.0, 28.0),
//     "Rubber",
//     if Focus::Sidebar == focus {Some(mouse)} else {None},
//     let_go,
//     if picker.get_col_rgba().is_none() { Color { r: 0.65, g: 0.8, b: 0.65, a: 1.0 } } else { LIGHTGRAY },
//     if picker.get_col_rgba().is_none() { Color { r: 0.60, g: 0.75, b: 0.60, a: 1.0 } } else { Color { r: 0.65, g: 0.65, b: 0.65, a: 1.0 } }
// ) {
//     picker.set_col(None);
// }
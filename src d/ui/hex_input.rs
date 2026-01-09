use std::{cell::RefCell, rc::Rc};

use super::*;
use colour_picker::Picker;

pub struct HexInput {
    pub rect: Rect,
    text: String,
    pub active: bool,
    default_col: Color,
    hover_col: Color,
    active_col: Color,
    pub selected_col: Option<[f32; 4]>,
    picker: Rc<RefCell<Picker>>,
    hover_possible: Rc<RefCell<bool>>
}

impl NewNoOut for HexInput {
    type InType = (Rect, Color, Color, Color, Rc<RefCell<Picker>>, Rc<RefCell<bool>>);
    fn new((rect, default_col, hover_col, active_col, picker, hover_possible): Self::InType, _handler: &mut GenHandler) -> Self {
        Self {
            rect,
            text: String::from("#"),
            active: false,
            default_col,
            hover_col,
            active_col,
            selected_col: None,
            picker,
            hover_possible
        }
    }
}

    // pub fn update_text(&mut self, col: Option<[f32; 4]>) { // THIS NEEDS TO GO IN UPDATE AND CHECK FOR USER INPUTS ITSELF
    //     if let Some(col) = col {
    //         self.text = ColSelection::format_rgba(col);
    //     } else {
    //         self.text = "None".to_string();
    //     }
    // }

// (mouse,        Focus::Sidebar == focus,  LastTouchFocus::TypeCol == last_touch_focus,  let_go      )
// (mouse: Vec2,  hover_possible: bool,     active_possible: bool,                        let_go: bool)

impl Node for HexInput {
    fn update(&mut self, user_inputs: &UserInputs, node: &NodeStore) {
        let mouse = user_inputs.mouse;
        let hover_possible = *self.hover_possible.borrow();
        let active_possible = user_inputs.last_touch_test(node);
        let let_go = user_inputs.left_let_go;

        let hovered = self.rect.contains(mouse) && hover_possible;

        if hovered && let_go {
            self.active = true;
            self.text = "#".to_string();
            while get_char_pressed().is_some() {}
        }

        if !active_possible {
            self.active = false
        }

        self.selected_col = None;

        // input
        if self.active {
            while let Some(c) = get_char_pressed() {
                if self.text.len() < 7 && c.is_ascii_hexdigit() {
                    self.text.push(c.to_ascii_uppercase());
                    if self.text.len() == 7 {
                        self.selected_col = Some(ColSelection::Rgba.col_from_hex_string(&self.text[1..]).to_rgba())
                    }
                }
            }

            if is_key_pressed(KeyCode::Backspace) {
                self.text.pop();
                if self.text.is_empty() {
                    self.text.push('#');
                }
            }
        }

        // draw box
        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            if self.active { self.active_col } else if hovered { self.hover_col } else { self.default_col },
        );

        draw_text(
            &self.text,
            self.rect.x + 6.0,
            self.rect.y + 22.0,
            22.0,
            BLACK,
        );
    }

    fn hit_detect(&mut self, pos: Vec2, node: &NodeStore) -> Vec<WeakNode> {
        if self.rect.contains(pos) {
            vec![node.get_weak()]
        } else {
            vec![]
        }
    }
}
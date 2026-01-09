use super::*;

pub struct HexInput {
    pub rect: Rect,
    text: String,
    pub active: bool,
    default_col: Color,
    hover_col: Color,
    active_col: Color,
    pub selected_col: Option<[f32; 4]>,
    node: WeakNode
}

impl New for HexInput {
    type InType = (Rect, Color, Color, Color);
    fn new((rect, default_col, hover_col, active_col): Self::InType, node: WeakNode) -> Self {
        Self {
            rect,
            text: String::from("#"),
            active: false,
            default_col,
            hover_col,
            active_col,
            selected_col: None,
            node
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
    fn update(&mut self, user_inputs: &UserInputs) {
        let mouse = user_inputs.mouse;
        let binding = self.node.upgrade().unwrap();
        let node = binding.borrow();
        let hover_possible = node.get_parent().borrow().contains_self(&user_inputs.hoverhold_focus);
        let active_possible = node.contains_self(&user_inputs.lasttouch_focus);
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

    fn hit_detect(&self, pos: Vec2) -> Vec<WeakNode> {
        if self.rect.contains(pos) {
            vec![self.node.clone()]
        } else {
            vec![]
        }
    }
}
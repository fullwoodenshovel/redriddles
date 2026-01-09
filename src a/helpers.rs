use macroquad::prelude::*;
use crate::colour::ColSelection;

pub fn mouse_vec() -> Vec2 {
    let (x, y) = mouse_position();
    vec2(x, y)
}

pub fn ui_button(rect: Rect, label: &str, mouse: Option<Vec2>, let_go: bool, default_col: Color, hover_col: Color) -> bool {
    let hovered = match mouse {
        Some(mouse) => rect.contains(mouse),
        None => false
    };

    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if hovered { hover_col } else { default_col },
    );

    draw_text(
        label,
        rect.x + 8.0,
        rect.y + rect.h * 0.7,
        18.0,
        BLACK,
    );

    hovered && let_go
}

pub fn col_button(rect: Rect, mouse: Option<Vec2>, let_go: bool, default_col: Color, hover_col: Color) -> bool {
    let hovered = match mouse {
        Some(mouse) => rect.contains(mouse),
        None => false
    };

    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if hovered { hover_col } else { default_col },
    );

    hovered && let_go
}

pub fn arr_to_macroquad(arr: [f32; 4]) -> Color {
    Color {
        r: arr[0],
        g: arr[1],
        b: arr[2],
        a: arr[3]
    }
}

pub struct HexInput {
    pub rect: Rect,
    text: String,
    pub active: bool,
    default_col: Color,
    hover_col: Color,
    active_col: Color
}

impl HexInput {
    pub fn new(rect: Rect, default_col: Color, hover_col: Color, active_col: Color) -> Self {
        Self {
            rect,
            text: String::from("#"),
            active: false,
            default_col,
            hover_col,
            active_col
        }
    }

    pub fn ui(&mut self, mouse: Vec2, hover_possible: bool, active_possible: bool, let_go: bool) -> Option<[f32; 4]> {
        let hovered = self.rect.contains(mouse) && hover_possible;

        if hovered && let_go {
            self.active = true;
            self.text = "#".to_string();
            while get_char_pressed().is_some() {}
        }

        if !active_possible {
            self.active = false
        }

        let mut result = None;

        // input
        if self.active {
            while let Some(c) = get_char_pressed() {
                if self.text.len() < 7 && c.is_ascii_hexdigit() {
                    self.text.push(c.to_ascii_uppercase());
                    if self.text.len() == 7 {
                        result = Some(ColSelection::Rgba.col_from_hex_string(&self.text[1..]).to_rgba())
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

        result
    }

    pub fn update_text(&mut self, col: Option<[f32; 4]>) {
        if let Some(col) = col {
            self.text = ColSelection::format_rgba(col);
        } else {
            self.text = "None".to_string();
        }
    }
}

use macroquad::prelude::*;

use crate::ui::{NodeStore, UserInputs};

pub fn mouse_vec() -> Vec2 {
    let (x, y) = mouse_position();
    vec2(x, y)
}

pub fn ui_button(rect: Rect, label: &str, default_col: Color, hover_col: Color, node: &NodeStore, user_inputs: &UserInputs) -> bool {
    let hovered = user_inputs.hover_test(node);
    let let_go = hovered && user_inputs.left_let_go && user_inputs.last_touch_test(node);

    raw_ui_button(rect, label, hovered, let_go, default_col, hover_col)
}

pub fn sub_ui_button(rect: Rect, label: &str, default_col: Color, hover_col: Color, node: &NodeStore, user_inputs: &UserInputs) -> bool {
    let hovered = user_inputs.hover_test(node) && rect.contains(user_inputs.mouse);
    let let_go = hovered && user_inputs.left_let_go && user_inputs.last_touch_test(node);

    raw_ui_button(rect, label, hovered, let_go, default_col, hover_col)
}

pub fn cut_text(text: &mut String, width: f32) {
    if measure_text(&text, None, 18, 1.0).width > width - 16.0 {
        *text = format!("...{text}");
        while text.len() > 3 && measure_text(&text, None, 18, 1.0).width > width - 16.0 {
            text.remove(3);
        }
    }
}

pub fn disabled_ui_button(rect: Rect, label: &str, col: Color) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        col,
    );
    
    draw_text(
        label,
        rect.x + 8.0,
        rect.y + rect.h * 0.7,
        18.0,
        BLACK,
    );
}

pub fn multiline_text(rect: Rect, label: &str) {
    for (index, line) in label.split("\n").enumerate() {
        draw_text(
            line,
            rect.x + 8.0,
            rect.y + index as f32 * 26.0,
            18.0,
            BLACK,
        );
    }    
}

pub fn raw_ui_button(rect: Rect, label: &str, hovered: bool, let_go: bool, default_col: Color, hover_col: Color) -> bool {
    disabled_ui_button(rect, label, if hovered { hover_col } else { default_col });

    hovered && let_go
}

pub fn col_button(rect: Rect, hovered: bool, let_go: bool, default_col: Color, hover_col: Color) -> bool {
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
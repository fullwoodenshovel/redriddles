
use macroquad::prelude::*;
use crate::{colour::ColSelection, node::{New, Node, UserInputs, WeakNode}};

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
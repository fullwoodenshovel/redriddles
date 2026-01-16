
use macroquad::prelude::*;

use crate::ui::{AppContextHandler, NodeStore};

pub fn mouse_vec() -> Vec2 {
    let (x, y) = mouse_position();
    vec2(x, y)
}

pub fn ui_button(rect: Rect, label: &str, default_col: Color, hover_col: Color, node: &NodeStore, ctx: &AppContextHandler) -> bool {
    let hovered = ctx.user_inputs.hover_test(node);
    let let_go = hovered && ctx.user_inputs.left_let_go && ctx.user_inputs.last_touch_test(node);

    raw_ui_button(rect, label, hovered, let_go, default_col, hover_col)
}

pub fn sub_ui_button(rect: Rect, label: &str, default_col: Color, hover_col: Color, node: &NodeStore, ctx: &AppContextHandler) -> bool {
    let hovered = ctx.user_inputs.hover_test(node) && rect.contains(ctx.user_inputs.mouse);
    let let_go = hovered && ctx.user_inputs.left_let_go && ctx.user_inputs.last_touch_test(node);

    raw_ui_button(rect, label, hovered, let_go, default_col, hover_col)
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
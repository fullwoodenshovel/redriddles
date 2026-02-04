
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
    let let_go = hovered && user_inputs.left_let_go && user_inputs.last_touch_test(node) && rect.contains(user_inputs.lasttouch_mouse);

    raw_ui_button(rect, label, hovered, let_go, default_col, hover_col)
}

pub fn cut_text(text: &mut String, width: f32) {
    if measure_text(text, None, 18, 1.0).width > width - 16.0 {
        *text = format!("...{text}");
        while text.len() > 3 && measure_text(text, None, 18, 1.0).width > width - 16.0 {
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

pub fn multiline_text(rect: Rect, text: &str, size: u16) {
    let mut lines = text.split("\n").map(|d| d.to_string()).collect::<Vec<_>>();
    let mut index = 0;
    while index != lines.len() {
        let cut = lines[index].split(" ").map(|d| d.to_string()).collect::<Vec<_>>();
        let mut j = cut.len();
        while measure_text(&cut[..j].join(" "), None, size, 1.0).width > rect.w && j != 0 {
            j -= 1;
        }

        if j == 0 {
            let mut result = lines[index].to_string();
            j = result.len();
            while measure_text(&result[..j], None, size, 1.0).width > rect.w && j != 1 {
                j -= 1;
            }
            if j != result.len() {
                lines.insert(index + 1, result.split_off(j));
                lines[index] = result;
            }
        } else if j != cut.len() {
            lines.insert(index + 1, cut[j..].join(" ").to_string());
            lines[index] = cut[..j].join(" ").to_string();
        }

        index += 1;
    }

    let step = size as f32 * 1.444;
    for (index, line) in lines.iter().enumerate() {
        draw_text(
            line,
            rect.x,
            rect.y + index as f32 * step,
            size as f32,
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

// inner_col = ENABLEDCOL
// border_col = DISABLEDCOL
// rect = Rect::new(150.0, 280.0, 300.0, 18.0)
#[allow(clippy::too_many_arguments)]
pub fn slider(
    inner_col: Color,
    border_col: Color,
    rect: Rect,
    label: &str,
    value: f32,
    min_value: f32,
    range: f32,
    user_inputs: &UserInputs,
    node: &NodeStore
) -> Option<f32> {
    draw_text(label, rect.x,rect.y - 10.0, 18.0, BLACK);
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, inner_col);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 4.0, border_col);

    let current_x = (value - min_value) / range * rect.w + rect.x;
    let y = rect.y + rect.h + 2.0;
    draw_triangle(vec2(current_x, y), vec2(current_x - 8.0, y + 8.0), vec2(current_x + 8.0, y + 8.0), BLACK);

    if user_inputs.left_mouse_down &&
        rect.contains(user_inputs.lasttouch_mouse) &&
        user_inputs.hoverhold_test(node)
    {
        Some(((user_inputs.mouse.x - rect.x) / rect.w * range + min_value).clamp(min_value, min_value + range))
    } else {
        None
    }
}

pub fn arr_to_macroquad(arr: [f32; 4]) -> Color {
    Color {
        r: arr[0],
        g: arr[1],
        b: arr[2],
        a: arr[3]
    }
}
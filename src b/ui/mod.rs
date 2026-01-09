pub use macroquad::miniquad::window::screen_size;
pub use macroquad::prelude::*;

use super::colour_picker;
pub use super::transform::*;
pub use super::colour::*;
pub use super::node::*;
pub mod draw_section;
pub mod draw;
pub mod export;
pub mod hex_input;
pub mod main;
pub mod settings;
pub mod sidebar;
pub mod topbar;


/// # Warning:
/// This function only recurs to the first child that returns. This is done in the same order that children are stored.
/// 
/// This is done to ensure a unique hitpoint and keep the resulting vector as a correct path down the tree.
/// 
/// The resulting Vec is from leaf to origin.
pub fn recur_hit_detect(node: &WeakNode, pos: Vec2) -> Vec<WeakNode> {
    let upgraded = node.upgrade().unwrap();
    let mut result = vec![];
    for child in &upgraded.borrow().children {
        result = child.borrow().get_self_dyn().hit_detect(pos);
        if !result.is_empty() {
            break
        }
    }
    result.push(node.clone());
    result
}






















// #[macroquad::main("Pixel Editor")]
// async fn main() {
//     loop {
//         if mouse_pressed {
//             if LastTouchFocus::TypeCol == last_touch_focus {
//                 hex_input.update_text(picker.get_col_rgba());
//             }

//             if hex_input.rect.contains(mouse) {
//                 last_touch_focus = if last_touch_focus == LastTouchFocus::TypeCol { LastTouchFocus::None } else { LastTouchFocus::TypeCol };
//             } else if save_rect.contains(mouse) {
//                 last_touch_focus = if last_touch_focus == LastTouchFocus::SaveCol { LastTouchFocus::None } else { LastTouchFocus::SaveCol };
//             } else if screen_picker_rect.contains(mouse) {
//                 last_touch_focus = if last_touch_focus == LastTouchFocus::Picker { LastTouchFocus::None } else { LastTouchFocus::Picker };
//             } else {
//                 last_touch_focus = LastTouchFocus::None;
//             }
//         }

//         // ---------------- DRAW AREA ----------------
//         if Focus::Draw == focus && !topbar.contains(mouse) && !sidebar.contains(mouse) {
//             // ZOOM / SCROLL
//             let (mut mx, mut my) = mouse_wheel();

//             if is_key_down(KeyCode::LeftControl) {
//                 if my != 0.0 {
//                     let zoom = 1.1_f32.powf(my);
//                     transform.scale_about(zoom, ScreenPos(mouse.x, mouse.y), 2.0, 80.0);
//                 }
//             } else {
//                 if is_key_down(KeyCode::LeftShift) {
//                     (mx, my) = (my, -mx);
//                 }
//                 transform.offset.0 += mx * 20.0;
//                 transform.offset.1 += my * 20.0;
//             }

//             // PAN
//             if is_mouse_button_down(MouseButton::Middle) {
//                 let delta = mouse_delta_position();
//                 transform.offset.0 += delta.x;
//                 transform.offset.1 += delta.y;
//             }

//             // GRID
//             if is_key_pressed(KeyCode::G) {
//                 if grid_lines {
//                     grid_lines = false;
//                 } else if crossboard {
//                     grid_lines = true;
//                     crossboard = false;
//                 } else {
//                     crossboard = true;
//                 }
//             }

//             if is_key_pressed(KeyCode::C) {
//                 crossboard = !crossboard;
//             }

//             // PAINT
//             if mouse_down && prev_touch_focus != LastTouchFocus::Picker {
//                 let world = transform.screen_to_world(&ScreenPos(mouse.x, mouse.y));
//                 if let Some(col) = picker.get_col_rgba() {
//                     if let Some(pixel) = Pixel::from_f32(world.0, world.1, col) {
//                         pixels.insert(pixel);
//                     }
//                 } else if let Some(pos) = world.as_i16() {
//                     pixels.remove(pos)
//                 }
//             }
//         }

//         // ---------------- DRAW WORLD ----------------
//         pixels.draw(&transform, grid_lines, crossboard);

//         // ---------------- TOP BAR ----------------
//         draw_rectangle(topbar.x, topbar.y, topbar.w, topbar.h, DARKGRAY);
//         draw_text("Pixel Editor (Zoom & Pan)", 10.0, 26.0, 22.0, WHITE);

//         // ---------------- SIDEBAR ----------------
//         draw_rectangle(sidebar.x, sidebar.y, sidebar.w, sidebar.h, GRAY);

//         for (y, cols) in saved_cols.iter_mut().enumerate() {
//             for (x, col) in cols.iter_mut().enumerate() {
//                 let x = 33.0 * x as f32 + 10.0;
//                 let y = 33.0 * y as f32 + 60.0;

//                 if col_button(
//                     Rect::new(x, y, 28.0, 28.0),
//                     if Focus::Sidebar == focus {Some(mouse)} else {None},
//                     last_touch_focus != prev_touch_focus || let_go,
//                     if last_touch_focus == LastTouchFocus::SaveCol { Color { r: 0.65, g: 0.8, b: 0.65, a: 1.0 } } else { LIGHTGRAY },
//                     if last_touch_focus == LastTouchFocus::SaveCol { Color { r: 0.60, g: 0.75, b: 0.60, a: 1.0 } } else { Color { r: 0.65, g: 0.65, b: 0.65, a: 1.0 } }
//                 ) {
//                     if prev_touch_focus == LastTouchFocus::SaveCol {
//                         *col = picker.get_col_rgba();
//                     } else {
//                         picker.set_col(*col);
//                     }
//                 }

//                 if let Some(col) = col {
//                     draw_rectangle(x + 4.0, y + 4.0, 20.0, 20.0, arr_to_macroquad(*col));
//                 }
//             }
//         }

//         // ---------------- COLOUR PICKER ----------------
//         if Focus::Picker == focus && mouse_down {
//             picker.detect(mouse, first_mouse_down);
//             hex_input.update_text(picker.get_col_rgba());
//         }
//         picker.draw();

//         if Focus::Picker == focus {
//             if is_key_pressed(KeyCode::Q) {
//                 picker.transfer_col(ColSelection::Rgba);
//             } else if is_key_pressed(KeyCode::W) {
//                 picker.transfer_col(ColSelection::Hsva);
//             } else if is_key_pressed(KeyCode::E) {
//                 picker.transfer_col(ColSelection::OkLab);
//             }
    
//             if is_key_pressed(KeyCode::A) {
//                 picker = picker.transfer_picker(PickerSelection::Circular);
//             } else if is_key_pressed(KeyCode::S) {
//                 picker = picker.transfer_picker(PickerSelection::Linear);
//             }
//         }

//         // ---------------- SAVE COLOUR ----------------
//         let mut flag = false;
//         if let Some(col) = picker.get_col_rgba() {
//             flag = true;
            
//             ui_button(
//                 save_rect,
//                 "Save colour",
//                 if Focus::Sidebar == focus {Some(mouse)} else {None},
//                 let_go,
//                 if last_touch_focus == LastTouchFocus::SaveCol { Color { r: 0.65, g: 0.8, b: 0.65, a: 1.0 } } else { LIGHTGRAY },
//                 if last_touch_focus == LastTouchFocus::SaveCol { Color { r: 0.60, g: 0.75, b: 0.60, a: 1.0 } } else { Color { r: 0.65, g: 0.65, b: 0.65, a: 1.0 } }
//             );

//             draw_rectangle(115.0, 380.0, 28.0, 28.0, LIGHTGRAY);
//             draw_rectangle(120.0, 385.0, 18.0, 18.0, arr_to_macroquad(col));
//         }

//         if flag && let Some(col) = hex_input.ui(mouse, Focus::Sidebar == focus, LastTouchFocus::TypeCol == last_touch_focus, let_go) {
//             picker.set_col(Some(col));
//         }

//         // ---------------- PICK COLOUR ----------------
//         ui_button(
//             screen_picker_rect,
//             "Pick colour",
//             if Focus::Sidebar == focus {Some(mouse)} else {None},
//             let_go,
//             if last_touch_focus == LastTouchFocus::Picker { Color { r: 0.65, g: 0.8, b: 0.65, a: 1.0 } } else { LIGHTGRAY },
//             if last_touch_focus == LastTouchFocus::Picker { Color { r: 0.60, g: 0.75, b: 0.60, a: 1.0 } } else { Color { r: 0.65, g: 0.65, b: 0.65, a: 1.0 } }
//         );

//         if last_touch_focus == LastTouchFocus::None && prev_touch_focus == LastTouchFocus::Picker && focus == Focus::Draw {
//             let world = transform.screen_to_world(&ScreenPos(mouse.x, mouse.y));
//             let world = world.as_i16();
//             if let Some(pos) = world && let Some(pixel) = pixels.get(pos) {
//                 picker.set_col(Some(pixel.col));
//             }
//         }
        
//         // ---------------- Rubber ----------------
//         if ui_button(
//             Rect::new(10.0, 482.0, 100.0, 28.0),
//             "Rubber",
//             if Focus::Sidebar == focus {Some(mouse)} else {None},
//             let_go,
//             if picker.get_col_rgba().is_none() { Color { r: 0.65, g: 0.8, b: 0.65, a: 1.0 } } else { LIGHTGRAY },
//             if picker.get_col_rgba().is_none() { Color { r: 0.60, g: 0.75, b: 0.60, a: 1.0 } } else { Color { r: 0.65, g: 0.65, b: 0.65, a: 1.0 } }
//         ) {
//             picker.set_col(None);
//         }

//         next_frame().await;
//     }
// }
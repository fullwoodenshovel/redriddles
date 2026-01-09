use std::{cell::RefCell, rc::Rc};

use crate::colour_picker::ColPicker;

use super::*;
use colour_picker::Picker;

pub struct DrawSection {
    pixels: PixelArray,
    transform: Transform,
    grid_lines: bool,
    crossboard: bool,
    picker: Rc<RefCell<Picker>>
}

impl NewNoOut for DrawSection {
    type InType = Rc<RefCell<Picker>>;
    fn new(picker: Self::InType, _handler: &mut GenHandler) -> Self {
        Self {
            pixels: PixelArray::default(),
            transform: Transform::new(screen_size()),
            grid_lines: false,
            crossboard: true,
            picker
        }
    }
}

impl Node for DrawSection {
    fn update(&mut self, user_inputs: &UserInputs, node: &NodeStore) {
        self.transform.window_dims = screen_size();

        let hoverhold = user_inputs.hoverhold_test(&node);
        let hover = user_inputs.hover_test(&node);


        // ---------------- DRAW AREA ----------------
        if hoverhold && hover {
            // ZOOM / SCROLL
            let (mut mx, mut my) = mouse_wheel();

            if is_key_down(KeyCode::LeftControl) {
                if my != 0.0 {
                    let zoom = 1.1_f32.powf(my);
                    self.transform.scale_about(zoom, ScreenPos(user_inputs.mouse.x, user_inputs.mouse.y), 2.0, 80.0);
                }
            } else {
                if is_key_down(KeyCode::LeftShift) {
                    (mx, my) = (my, -mx);
                }
                self.transform.offset.0 += mx * 20.0;
                self.transform.offset.1 += my * 20.0;
            }

            // PAN
            if is_mouse_button_down(MouseButton::Middle) {
                let delta = mouse_delta_position();
                self.transform.offset.0 += delta.x;
                self.transform.offset.1 += delta.y;
            }

            // GRID
            if is_key_pressed(KeyCode::G) {
                if self.grid_lines {
                    self.grid_lines = false;
                } else if self.crossboard {
                    self.grid_lines = true;
                    self.crossboard = false;
                } else {
                    self.crossboard = true;
                }
            }

            if is_key_pressed(KeyCode::C) {
                self.crossboard = !self.crossboard;
            }

            // PAINT
            if user_inputs.left_mouse_down && hoverhold && hover {
                let world = self.transform.screen_to_world(&ScreenPos(user_inputs.mouse.x, user_inputs.mouse.y));
                if let Some(col) = self.picker.borrow_mut().get_col_rgba() {
                    if let Some(pixel) = Pixel::from_f32(world.0, world.1, col) {
                        self.pixels.insert(pixel);
                    }
                } else if let Some(pos) = world.as_i16() {
                    self.pixels.remove(pos)
                }
            }
        }

        // ---------------- DRAW WORLD ----------------
        self.pixels.draw(&self.transform, self.grid_lines, self.crossboard);
    }

    fn hit_detect(&mut self, _pos: Vec2, node: &NodeStore) -> Vec<WeakNode> {
        vec![node.get_weak()]
    }
}
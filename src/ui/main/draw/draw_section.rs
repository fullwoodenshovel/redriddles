use crate::colour_picker::ColPicker;

use super::*;
use colour_picker::Picker;

pub struct DrawSection {
    transform: Transform,
    grid_lines: bool,
    crossboard: bool,
    line_start: Option<[i16; 2]>,
    prev_touch: Option<[i16; 2]>,
}

impl New for DrawSection {
    fn new(handler: &mut GenHandler) -> Self {
        handler.push_data(PixelArray::default());

        Self {
            transform: Transform::new(screen_size()),
            grid_lines: false,
            crossboard: true,
            line_start: None,
            prev_touch: None,
        }
    }
}

impl Node for DrawSection {
    fn update(&mut self, ctx: &mut AppContextHandler, node: &NodeStore) {
        self.transform.window_dims = screen_size();

        let hoverhold = ctx.user_inputs.hoverhold_test(node);
        let hover = ctx.user_inputs.hover_test(node);

        // ---------------- DRAW WORLD ----------------
        ctx.store.get_mut::<PixelArray>().draw(&self.transform, self.grid_lines, self.crossboard);

        let prev_touch = self.prev_touch;
        self.prev_touch = None;

        // ---------------- DRAW AREA ----------------
        if let ShortcutInstruction::ChangeDrawState(state) = ctx.user_inputs.pressed_instruction {
            if *ctx.store.get::<DrawState>() == state {
                ctx.store.overwrite(DrawState::Draw);
            } else {
                ctx.store.overwrite(state);
            }
        } else if let ShortcutInstruction::Eraser = ctx.user_inputs.pressed_instruction {
            ctx.store.get_mut::<Picker>().set_col(None);
        }
        if hoverhold && hover {
            // ZOOM / SCROLL
            let (mut mx, mut my) = mouse_wheel();
            if cfg!(target_os = "windows") {                
                mx /= 120.0;
                my /= 120.0;
            }

            if is_key_down(KeyCode::LeftControl) {
                if my != 0.0 {
                    let zoom = 1.1_f32.powf(my);
                    self.transform.scale_about(zoom, ScreenPos(ctx.user_inputs.mouse.x, ctx.user_inputs.mouse.y), 2.0, 80.0);
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
                let delta = ctx.user_inputs.mouse - ctx.user_inputs.prev_mouse;
                self.transform.offset.0 += delta.x;
                self.transform.offset.1 += delta.y;
            }

            // GRID
            if ctx.user_inputs.instruction_pressed(ShortcutInstruction::ToggleGrid) {
                if self.grid_lines {
                    self.grid_lines = false;
                } else if self.crossboard {
                    self.grid_lines = true;
                    self.crossboard = false;
                } else {
                    self.crossboard = true;
                }
            }

            let state = ctx.store.get::<DrawState>();

            // PAINT
            match state {
                DrawState::Draw => {
                    if ctx.user_inputs.left_mouse_down && hoverhold && hover {
                        let world = self.transform.screen_to_world(&ScreenPos(ctx.user_inputs.mouse.x, ctx.user_inputs.mouse.y));
                        let col = ctx.store.get_mut::<Picker>().get_col_rgba();
                        if let Some(pos) = world.as_i16() {
                            if let Some(start) = prev_touch {
                                ctx.store.get_mut::<PixelArray>().line(start, world.as_i16().unwrap(), col);
                            } else {
                                match col {
                                    Some(col) => ctx.store.get_mut::<PixelArray>().insert(Pixel { pos, col }),
                                    None => ctx.store.get_mut::<PixelArray>().remove(pos),
                                }
                            }
                            self.prev_touch = Some(pos);
                        }
                    }
                },
                DrawState::Picker => {
                    if ctx.user_inputs.left_let_go && hoverhold && hover {
                        if let Some(&pixel) = ctx.store.get::<PixelArray>().get_at_mouse(ctx.user_inputs.mouse, &self.transform) {
                            ctx.store.get_mut::<Picker>().set_col(Some(pixel.col));
                        }
                        ctx.store.overwrite(DrawState::Draw);
                    }
                },
                DrawState::Fill => {
                    if ctx.user_inputs.left_mouse_down && hoverhold && hover && let Some(pos) = self.transform.get_int_pos(ctx.user_inputs.mouse) {
                        let col = ctx.store.get_mut::<Picker>().get_col_rgba();
                        ctx.store.get_mut::<PixelArray>().fill(pos, col);
                        ctx.store.overwrite(DrawState::Draw);
                    }
                },
                DrawState::Line => {
                    if ctx.user_inputs.left_mouse_pressed && hoverhold && hover {
                        self.line_start = self.transform.get_int_pos(ctx.user_inputs.mouse);
                    } else if ctx.user_inputs.left_let_go {
                        if hoverhold && hover && let Some(start) = self.line_start && let Some(end) = self.transform.get_int_pos(ctx.user_inputs.mouse) {
                            let col = ctx.store.get_mut::<Picker>().get_col_rgba();
                            ctx.store.get_mut::<PixelArray>().line(start, end, col);
                        }
                        self.line_start = None
                    } else if ctx.user_inputs.left_mouse_down && hoverhold && hover && let Some(start) = self.line_start && let Some(end) = self.transform.get_int_pos(ctx.user_inputs.mouse) {
                        let mut pixels = PixelArray::default();
                        pixels.line(start, end, if let Some(col) = ctx.store.get_mut::<Picker>().get_col_rgba() { Some(col) } else { Some([0.0, 0.0, 0.0, 0.6]) });
                        pixels.draw(&self.transform, false, false);
                    }
                }
            }
        }
    }

    fn hit_detect(&mut self, _pos: Vec2, node: &NodeStore, _store: &mut Store) -> Vec<WeakNode> {
        vec![node.get_weak()]
    }
}
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use macroquad::miniquad::window::screen_size;
use macroquad::prelude::*;

use super::colour_picker::{picker::Circular, Picker, PickerSelection, ColPicker};
use super::transform::*;
use super::colour::*;
use super::helpers::*;


pub struct UserInputs {
    pub hoverhold_focus: Option<WeakRef<dyn Node>>,
    pub lasttouch_focus: Option<WeakRef<dyn Node>>,
    pub mouse: Vec2,
    pub left_mouse_pressed: bool,
    pub right_mouse_pressed: bool,
    pub left_mouse_down: bool,
    pub right_mouse_down: bool,
    pub left_let_go: bool,
    pub right_let_go: bool,
    origin_node: WeakRef<dyn Node>,
}

impl UserInputs {
    fn new(origin_node: WeakRef<dyn Node>) -> Self {
        Self {
            hoverhold_focus: None,
            lasttouch_focus: None,
            mouse: Vec2::splat(0.0),
            left_mouse_pressed: false,
            right_mouse_pressed: false,
            left_mouse_down: false,
            right_mouse_down: false,
            left_let_go: false,
            right_let_go: false,
            origin_node, 
        }
    }

    fn update(&mut self) {
        self.mouse = mouse_vec();
        self.left_mouse_pressed = is_mouse_button_pressed(MouseButton::Left);
        self.right_mouse_pressed = is_mouse_button_pressed(MouseButton::Right);
        self.left_mouse_down = is_mouse_button_down(MouseButton::Left);
        self.right_mouse_down = is_mouse_button_down(MouseButton::Right);
        self.left_let_go = is_mouse_button_released(MouseButton::Left);
        self.right_let_go = is_mouse_button_released(MouseButton::Right);

        if self.left_mouse_pressed || !self.left_mouse_down {
            if let Some(node) = self.origin_node.upgrade() {
                self.hoverhold_focus = Some(node.borrow_mut().hit_detect(self.mouse));
                if self.left_mouse_pressed {
                    self.lasttouch_focus = self.hoverhold_focus.clone();
                }
            } else {
                eprintln!("ERROR 1");
            }
        }
    }
}

pub type StrongRef<T> = Rc<RefCell<T>>;
pub type WeakRef<T> = Weak<RefCell<T>>;

pub trait Node {
    fn update(&mut self, user_inputs: &UserInputs);
    fn hit_detect(&mut self, pos: Vec2) -> WeakRef<dyn Node>;
}

trait Toggleable: Node {
    fn turn_on(&mut self);
    fn turn_off(&mut self);
}

struct Main {
    status: usize, // 0 is draw, 1 is settings, 2 is export
    backcols: [Color; 3],
    children: [StrongRef<dyn Toggleable>; 3],
}

impl Default for Main {
    fn default() -> Self {
        Self {
            status: 0,
            backcols: [Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }; 3],
            children: [
                Rc::new(RefCell::new(Draw::default())),
                Rc::new(RefCell::new(Settings::default())),
                Rc::new(RefCell::new(Export::default()))
            ]
        }
    }
}

impl Node for Main {
    fn update(&mut self, user_inputs: &UserInputs) {
        clear_background(self.backcols[self.status]);
        self.children[self.status].borrow_mut().update(user_inputs);
    }

    fn hit_detect(&mut self, pos: Vec2) -> WeakRef<dyn Node> {
        self.children[self.status].borrow_mut().hit_detect(pos)
    }
}

#[derive(Default)]
struct Settings;
impl Node for Settings {fn update(&mut self, _user_inputs: &UserInputs) {} fn hit_detect(&mut self, _pos: Vec2) -> WeakRef<dyn Node> {unimplemented!()}}
impl Toggleable for Settings {fn turn_on(&mut self) {} fn turn_off(&mut self) {}}
#[derive(Default)]
struct Export;
impl Node for Export {fn update(&mut self, _user_inputs: &UserInputs) {} fn hit_detect(&mut self, _pos: Vec2) -> WeakRef<dyn Node> {unimplemented!()}}
impl Toggleable for Export {fn turn_on(&mut self) {} fn turn_off(&mut self) {}}

struct Draw {
    topbar: StrongRef<Topbar>,
    sidebar: StrongRef<Sidebar>,
    draw_section: StrongRef<DrawSection>
}

impl Default for Draw {
    fn default() -> Self {
        Self {
            topbar: Rc::new(RefCell::new(Topbar::default())),
            sidebar: Rc::new(RefCell::new(Sidebar::default())),
            draw_section: Rc::new(RefCell::new(DrawSection::default()))
        }
    }
}

impl Node for Draw {
    fn update(&mut self, user_inputs: &UserInputs) {
        self.topbar.borrow_mut().update(user_inputs);
        self.sidebar.borrow_mut().update(user_inputs);
        self.draw_section.borrow_mut().update(user_inputs);
    }

    fn hit_detect(&mut self, pos: Vec2) -> WeakRef<dyn Node> {
        todo!()
    }
}

impl Toggleable for Draw {
    fn turn_on(&mut self) {
        
    }

    fn turn_off(&mut self) {
        
    }
}

struct Topbar {
    start_pos: Vec2,
    size: Vec2
}

impl Default for Topbar {
    fn default() -> Self {
        Self {
            start_pos: Vec2::new(0.0, 0.0),
            size: Vec2::new(screen_width(), 40.0)
        }
    }
}

impl Topbar {
    fn bounding_box(&self) -> Rect {
        Rect::new(self.start_pos.x, self.start_pos.y, self.size.x, self.size.y)
    }
}

impl Node for Topbar {
    fn update(&mut self, _user_inputs: &UserInputs) {
        self.size.x = screen_width();
    }

    fn hit_detect(&mut self, _pos: Vec2) -> WeakRef<dyn Node> {
        unimplemented!()
    }
}

struct Sidebar {
    start_pos: Vec2,
    size: Vec2,
    picker: Picker,
    saved_cols: Vec<[Option<[f32; 4]>; 4]>,
    hex_input: HexInput,
    save_rect: Rect,
    screen_picker_rect: Rect,
}

impl Default for Sidebar {
    fn default() -> Self {
        Self {
            start_pos: Vec2::new(0.0, 40.0),
            size: Vec2::new(150.0, screen_height() - 40.0),
            picker: Picker::Circular(Circular::new(50.0, 10.0, [10.0, 270.0], 16.0, ColSelection::Hsva)),
            saved_cols: vec![[None; 4]; 6],
            hex_input: HexInput::new(
                Rect::new(10.0, 414.0, 100.0, 28.0),
                LIGHTGRAY,
                Color { r: 0.65, g: 0.65, b: 0.65, a: 1.0 },
                Color { r: 0.65, g: 0.8, b: 0.65, a: 1.0 }
            ),
            save_rect: Rect::new(10.0, 380.0, 100.0, 28.0),
            screen_picker_rect: Rect::new(10.0, 448.0, 100.0, 28.0),
        }
    }
}

impl Node for Sidebar {
    fn update(&mut self, user_inputs: &UserInputs) {
        self.size.y = screen_height() - 40.0;
    }

    fn hit_detect(&mut self, pos: Vec2) -> WeakRef<dyn Node> {
        todo!()
    }
}

impl Node for HexInput {
    fn update(&mut self, user_inputs: &UserInputs) {
        self.
    }
}

struct DrawSection {
    pixels: PixelArray,
    transform: Transform,
    grid_lines: bool,
    crossboard: bool,
}

impl Default for DrawSection {
    fn default() -> Self {
        Self {
            pixels: PixelArray::default(),
            transform: Transform::new(screen_size()),
            grid_lines: false,
            crossboard: true,
        }
    }
}

impl Node for DrawSection {
    fn update(&mut self, user_inputs: &UserInputs) {
        self.transform.window_dims = screen_size();
    }

    fn hit_detect(&mut self, pos: Vec2) -> WeakRef<dyn Node> {
        todo!()
    }
}





















#[macroquad::main("Pixel Editor")]
async fn main() {
    loop {
        if mouse_pressed {
            if LastTouchFocus::TypeCol == last_touch_focus {
                hex_input.update_text(picker.get_col_rgba());
            }

            if hex_input.rect.contains(mouse) {
                last_touch_focus = if last_touch_focus == LastTouchFocus::TypeCol { LastTouchFocus::None } else { LastTouchFocus::TypeCol };
            } else if save_rect.contains(mouse) {
                last_touch_focus = if last_touch_focus == LastTouchFocus::SaveCol { LastTouchFocus::None } else { LastTouchFocus::SaveCol };
            } else if screen_picker_rect.contains(mouse) {
                last_touch_focus = if last_touch_focus == LastTouchFocus::Picker { LastTouchFocus::None } else { LastTouchFocus::Picker };
            } else {
                last_touch_focus = LastTouchFocus::None;
            }
        }

        // ---------------- DRAW AREA ----------------
        if Focus::Draw == focus && !topbar.contains(mouse) && !sidebar.contains(mouse) {
            // ZOOM / SCROLL
            let (mut mx, mut my) = mouse_wheel();

            if is_key_down(KeyCode::LeftControl) {
                if my != 0.0 {
                    let zoom = 1.1_f32.powf(my);
                    transform.scale_about(zoom, ScreenPos(mouse.x, mouse.y), 2.0, 80.0);
                }
            } else {
                if is_key_down(KeyCode::LeftShift) {
                    (mx, my) = (my, -mx);
                }
                transform.offset.0 += mx * 20.0;
                transform.offset.1 += my * 20.0;
            }

            // PAN
            if is_mouse_button_down(MouseButton::Middle) {
                let delta = mouse_delta_position();
                transform.offset.0 += delta.x;
                transform.offset.1 += delta.y;
            }

            // GRID
            if is_key_pressed(KeyCode::G) {
                if grid_lines {
                    grid_lines = false;
                } else if crossboard {
                    grid_lines = true;
                    crossboard = false;
                } else {
                    crossboard = true;
                }
            }

            if is_key_pressed(KeyCode::C) {
                crossboard = !crossboard;
            }

            // PAINT
            if mouse_down && prev_touch_focus != LastTouchFocus::Picker {
                let world = transform.screen_to_world(&ScreenPos(mouse.x, mouse.y));
                if let Some(col) = picker.get_col_rgba() {
                    if let Some(pixel) = Pixel::from_f32(world.0, world.1, col) {
                        pixels.insert(pixel);
                    }
                } else if let Some(pos) = world.as_i16() {
                    pixels.remove(pos)
                }
            }
        }

        // ---------------- DRAW WORLD ----------------
        pixels.draw(&transform, grid_lines, crossboard);

        // ---------------- TOP BAR ----------------
        draw_rectangle(topbar.x, topbar.y, topbar.w, topbar.h, DARKGRAY);
        draw_text("Pixel Editor (Zoom & Pan)", 10.0, 26.0, 22.0, WHITE);

        // ---------------- SIDEBAR ----------------
        draw_rectangle(sidebar.x, sidebar.y, sidebar.w, sidebar.h, GRAY);

        for (y, cols) in saved_cols.iter_mut().enumerate() {
            for (x, col) in cols.iter_mut().enumerate() {
                let x = 33.0 * x as f32 + 10.0;
                let y = 33.0 * y as f32 + 60.0;

                if col_button(
                    Rect::new(x, y, 28.0, 28.0),
                    if Focus::Sidebar == focus {Some(mouse)} else {None},
                    last_touch_focus != prev_touch_focus || let_go,
                    if last_touch_focus == LastTouchFocus::SaveCol { Color { r: 0.65, g: 0.8, b: 0.65, a: 1.0 } } else { LIGHTGRAY },
                    if last_touch_focus == LastTouchFocus::SaveCol { Color { r: 0.60, g: 0.75, b: 0.60, a: 1.0 } } else { Color { r: 0.65, g: 0.65, b: 0.65, a: 1.0 } }
                ) {
                    if prev_touch_focus == LastTouchFocus::SaveCol {
                        *col = picker.get_col_rgba();
                    } else {
                        picker.set_col(*col);
                    }
                }

                if let Some(col) = col {
                    draw_rectangle(x + 4.0, y + 4.0, 20.0, 20.0, arr_to_macroquad(*col));
                }
            }
        }

        // ---------------- COLOUR PICKER ----------------
        if Focus::Picker == focus && mouse_down {
            picker.detect(mouse, first_mouse_down);
            hex_input.update_text(picker.get_col_rgba());
        }
        picker.draw();

        if Focus::Picker == focus {
            if is_key_pressed(KeyCode::Q) {
                picker.transfer_col(ColSelection::Rgba);
            } else if is_key_pressed(KeyCode::W) {
                picker.transfer_col(ColSelection::Hsva);
            } else if is_key_pressed(KeyCode::E) {
                picker.transfer_col(ColSelection::OkLab);
            }
    
            if is_key_pressed(KeyCode::A) {
                picker = picker.transfer_picker(PickerSelection::Circular);
            } else if is_key_pressed(KeyCode::S) {
                picker = picker.transfer_picker(PickerSelection::Linear);
            }
        }

        // ---------------- SAVE COLOUR ----------------
        let mut flag = false;
        if let Some(col) = picker.get_col_rgba() {
            flag = true;
            
            ui_button(
                save_rect,
                "Save colour",
                if Focus::Sidebar == focus {Some(mouse)} else {None},
                let_go,
                if last_touch_focus == LastTouchFocus::SaveCol { Color { r: 0.65, g: 0.8, b: 0.65, a: 1.0 } } else { LIGHTGRAY },
                if last_touch_focus == LastTouchFocus::SaveCol { Color { r: 0.60, g: 0.75, b: 0.60, a: 1.0 } } else { Color { r: 0.65, g: 0.65, b: 0.65, a: 1.0 } }
            );

            draw_rectangle(115.0, 380.0, 28.0, 28.0, LIGHTGRAY);
            draw_rectangle(120.0, 385.0, 18.0, 18.0, arr_to_macroquad(col));
        }

        if flag && let Some(col) = hex_input.ui(mouse, Focus::Sidebar == focus, LastTouchFocus::TypeCol == last_touch_focus, let_go) {
            picker.set_col(Some(col));
        }

        // ---------------- PICK COLOUR ----------------
        ui_button(
            screen_picker_rect,
            "Pick colour",
            if Focus::Sidebar == focus {Some(mouse)} else {None},
            let_go,
            if last_touch_focus == LastTouchFocus::Picker { Color { r: 0.65, g: 0.8, b: 0.65, a: 1.0 } } else { LIGHTGRAY },
            if last_touch_focus == LastTouchFocus::Picker { Color { r: 0.60, g: 0.75, b: 0.60, a: 1.0 } } else { Color { r: 0.65, g: 0.65, b: 0.65, a: 1.0 } }
        );

        if last_touch_focus == LastTouchFocus::None && prev_touch_focus == LastTouchFocus::Picker && focus == Focus::Draw {
            let world = transform.screen_to_world(&ScreenPos(mouse.x, mouse.y));
            let world = world.as_i16();
            if let Some(pos) = world && let Some(pixel) = pixels.get(pos) {
                picker.set_col(Some(pixel.col));
            }
        }
        
        // ---------------- Rubber ----------------
        if ui_button(
            Rect::new(10.0, 482.0, 100.0, 28.0),
            "Rubber",
            if Focus::Sidebar == focus {Some(mouse)} else {None},
            let_go,
            if picker.get_col_rgba().is_none() { Color { r: 0.65, g: 0.8, b: 0.65, a: 1.0 } } else { LIGHTGRAY },
            if picker.get_col_rgba().is_none() { Color { r: 0.60, g: 0.75, b: 0.60, a: 1.0 } } else { Color { r: 0.65, g: 0.65, b: 0.65, a: 1.0 } }
        ) {
            picker.set_col(None);
        }

        next_frame().await;
    }
}
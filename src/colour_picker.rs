use std::ops::{Deref, DerefMut};

use macroquad::{camera::{Camera2D, set_camera, set_default_camera}, color::{Color, WHITE}, math::{Rect, Vec2}, texture::{DrawTextureParams, FilterMode, RenderTarget, draw_texture_ex, render_target}, window::clear_background};
use enum_dispatch::enum_dispatch;

use crate::{colour::ColSelection, node::{GenHandler, New, Node, NodeStore, AppContextHandler, Store, WeakNode, ShortcutInstruction}};
mod circular;
mod linear;

pub mod picker {
    pub use super::circular::Circular;
    pub use super::linear::Linear;
}

#[enum_dispatch(ColPicker)]
pub enum PickerEnum {
    Circular(picker::Circular),
    Linear(picker::Linear)
}

pub struct Picker {
    picker: Option<PickerEnum>
}

impl Picker {
    pub fn new(picker: PickerEnum) -> Self {
        Self { picker: Some(picker) }
    }

    pub fn new_circular(radius: f32, width: f32, offset: [f32;2], padding: f32, coltype: ColSelection) -> Self {
        Self { picker: Some(PickerEnum::Circular(picker::Circular::new(radius, width, offset, padding, coltype))) }
    }

    pub fn new_linear(height: f32, width: f32, offset: [f32;2], padding: f32, coltype: ColSelection) -> Self {
        Self { picker: Some(PickerEnum::Linear(picker::Linear::new(height, width, offset, padding, coltype))) }
    }

    pub fn transfer_picker(&mut self, pickertype: PickerSelection) {
        self.picker = Some(self.picker.take().unwrap().transfer_picker(pickertype));
    }
}

impl Deref for Picker {
    type Target = PickerEnum;
    fn deref(&self) -> &Self::Target {
        self.picker.as_ref().unwrap()
    }
}

impl DerefMut for Picker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.picker.as_mut().unwrap()
    }
}

pub struct PickerNode;

impl Node for PickerNode {
    fn update(&mut self, ctx: &mut AppContextHandler, node: &NodeStore) {
        if let ShortcutInstruction::ChangePickerType(pickertype) = ctx.user_inputs.pressed_instruction {
            let picker = ctx.store.get_mut::<Picker>();
            match pickertype {
                ColSelection::Rgba => {
                    picker.transfer_col(ColSelection::Rgba);
                    picker.transfer_picker(PickerSelection::Linear);
                },
                ColSelection::Hsva => {
                    picker.transfer_col(ColSelection::Hsva);
                    picker.transfer_picker(PickerSelection::Circular);
                },
                ColSelection::OkLab => {
                    picker.transfer_col(ColSelection::OkLab);
                    picker.transfer_picker(PickerSelection::Linear);
                }
            }
        }

        if ctx.user_inputs.left_mouse_down && ctx.user_inputs.hoverhold_test(node) {
            ctx.store.get_mut::<Picker>().detect(ctx.user_inputs.mouse, ctx.user_inputs.hoverhold_mouse);
        }
        ctx.store.get_mut::<Picker>().draw();

    }

    fn hit_detect(&mut self, pos: Vec2, node: &NodeStore, handler: &mut Store) -> Vec<WeakNode> {
        if handler.get_mut::<Picker>().bounding_box().contains(pos) {
            vec![node.get_weak()]
        } else {
            vec![]
        }
    }
}

impl New for PickerNode {
    fn new(handler: &mut GenHandler) -> Self {
        handler.push_data(Picker::new_circular(50.0, 10.0, [10.0, 270.0], 16.0, ColSelection::Hsva));
        Self {
        }
    }
}

pub enum PickerSelection {
    Circular,
    Linear
}

#[enum_dispatch]
pub trait ColPicker {
    fn bounding_box(&self) -> Rect;
    fn detect(&mut self, mouse: Vec2, first_mouse_down: Vec2);
    fn draw(&mut self);
    fn set_col(&mut self, col: Option<[f32; 4]>);
    fn transfer_col(&mut self, coltype: ColSelection);
    fn transfer_picker(self, pickertype: PickerSelection) -> PickerEnum;
    fn get_col_rgba(&mut self) -> Option<[f32; 4]>;
}


pub struct RedrawGuard<'a> {
    surface: &'a mut SurfaceCache
}

impl<'a> Drop for RedrawGuard<'a> {
    fn drop(&mut self) {
        set_default_camera();
        self.surface.render();
    }
}

pub struct SurfaceCache {
    dirty: bool,
    surface: RenderTarget,
    offset: Vec2,
    size: Vec2
}

impl SurfaceCache {
    pub fn new(rect: Rect) -> Self {
        let surface = render_target(rect.w as u32, rect.h as u32);
        surface.texture.set_filter(FilterMode::Nearest);
        Self {
            dirty: true,
            surface,
            offset: Vec2::new(rect.x, rect.y),
            size: Vec2::new(rect.w, rect.h)
        }
    }

    pub fn invalidate(&mut self) {
        self.dirty = true;
    }

    fn render(&mut self) {
        draw_texture_ex(
            &self.surface.texture,
            self.offset.x,
            self.offset.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.size),
                ..Default::default()
            }
        );
    }

    pub fn redraw(&mut self) -> Option<RedrawGuard<'_>> {
        if !self.dirty {
            self.render();
            return None
        }

        self.dirty = false;

        set_camera(&Camera2D {
            target: self.size / 2.0,
            zoom: Vec2::new(2.0, 2.0) / self.size,
            render_target: Some(self.surface.clone()),
            ..Default::default()
        });

        clear_background(Color::new(0.0, 0.0, 0.0, 0.0));

        Some(RedrawGuard { surface: self })
    }
}
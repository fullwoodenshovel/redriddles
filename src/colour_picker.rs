use std::ops::{Deref, DerefMut};

use macroquad::math::{Rect, Vec2};
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
        if let ShortcutInstruction::ChangePickerType(pickertype) = ctx.user_inputs.pressed_shortcut {
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
        ctx.store.get::<Picker>().draw();

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
    fn draw(&self);
    fn set_col(&mut self, col: Option<[f32; 4]>);
    fn transfer_col(&mut self, coltype: ColSelection);
    fn transfer_picker(self, pickertype: PickerSelection) -> PickerEnum;
    fn get_col_rgba(&mut self) -> Option<[f32; 4]>;
}

use macroquad::math::{Rect, Vec2};
use enum_dispatch::enum_dispatch;

use crate::{colour::{Col, ColSelection}, node::{New, Node, UserInputs, WeakNode}};
mod circular;
mod linear;

pub mod picker {
    pub use super::circular::Circular;
    pub use super::linear::Linear;
}

#[enum_dispatch(ColPicker)]
pub enum Picker {
    Circular(picker::Circular),
    Linear(picker::Linear)
}

pub struct PickerNode {
    picker: Option<Picker>,
    node: WeakNode
}

impl Node for PickerNode {
    fn update(&mut self, user_inputs: &UserInputs) {
        if user_inputs.hoverhold_test(&self.node.upgrade().unwrap()) {
            self.get_picker_mut().detect(user_inputs.mouse, user_inputs.hoverhold_mouse);
        }
        self.get_picker().draw();
    }

    fn hit_detect(&self, pos: Vec2) -> Vec<WeakNode> {
        if self.get_picker().bounding_box().contains(pos) {
            vec![self.node.clone()]
        } else {
            vec![]
        }
    }
}

impl New for PickerNode {
    type InType = Picker;
    fn new(picker: Self::InType, node: WeakNode) -> Self {
        Self {
            picker: Some(picker),
            node
        }
    }
}

impl PickerNode {
    fn get_picker(&self) -> &Picker {
        self.picker.as_ref().unwrap()
    }

    fn get_picker_mut(&mut self) -> &mut Picker {
        self.picker.as_mut().unwrap()
    }

    pub fn set_col(&mut self, col: Option<[f32; 4]>) {
        self.get_picker_mut().set_col(col);
    }

    pub fn transfer_col(&mut self, coltype: ColSelection) {
        self.get_picker_mut().transfer_col(coltype);
    }

    pub fn transfer_picker(&mut self, pickertype: PickerSelection) {
        let picker = self.picker.take().unwrap();
        self.picker = Some(picker.transfer_picker(pickertype));
    }

    pub fn get_col_rgba(&mut self) -> Option<[f32; 4]> {
        self.get_picker_mut().get_col_rgba()
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
    fn transfer_picker(self, pickertype: PickerSelection) -> Picker;
    fn get_col_rgba(&mut self) -> Option<[f32; 4]>;
}

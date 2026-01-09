use macroquad::math::{Rect, Vec2};
use enum_dispatch::enum_dispatch;

use crate::{colour::ColSelection, node::{New, Node, WeakNode}};
mod circular;
mod linear;

pub mod picker {
    pub use super::circular::Circular;
    pub use super::linear::Linear;
}

pub struct Picker {
    picker: Box<dyn ColPicker>,
    node: WeakNode
}

impl New for Picker {
    type InType = Box<dyn ColPicker>;
    fn new(picker: Self::InType, node: WeakNode) -> Self {
        Self {
            picker,
            node 
        }
    }
}

impl Node for Picker {
    fn update(&mut self, user_inputs: &crate::node::UserInputs) {
        todo!()
    }

    fn hit_detect(&mut self, pos: Vec2) -> Vec<WeakNode> {
        todo!()
    }
}

pub enum PickerSelection {
    Circular,
    Linear
}

pub trait ColPicker {
    fn bounding_box(&self) -> Rect;
    fn detect(&mut self, mouse: Vec2, first_mouse_down: Vec2);
    fn draw(&self);
    fn set_col(&mut self, col: Option<[f32; 4]>);
    fn transfer_col(&mut self, coltype: ColSelection);
    fn transfer_picker(self, pickertype: PickerSelection) -> Box<dyn ColPicker>;
    fn get_col_rgba(&mut self) -> Option<[f32; 4]>;
}

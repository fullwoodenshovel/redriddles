use std::{cell::RefCell, ops::{Deref, DerefMut}, rc::Rc};

use macroquad::{input::{KeyCode, is_key_pressed}, math::{Rect, Vec2}};
use enum_dispatch::enum_dispatch;

use crate::{colour::ColSelection, node::{GenHandler, New, Node, NodeStore, UserInputs, WeakNode}};
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

pub struct PickerNode {
    picker: Rc<RefCell<Picker>>
}

impl Node for PickerNode {
    fn update(&mut self, user_inputs: &UserInputs, node: &NodeStore) {
        if user_inputs.hoverhold_test(node) {
            let mut picker = self.picker.borrow_mut();
            picker.detect(user_inputs.mouse, user_inputs.hoverhold_mouse);

            if is_key_pressed(KeyCode::Q) {
                picker.transfer_col(ColSelection::Rgba);
            } else if is_key_pressed(KeyCode::W) {
                picker.transfer_col(ColSelection::Hsva);
            } else if is_key_pressed(KeyCode::E) {
                picker.transfer_col(ColSelection::OkLab);
            }
    
            if is_key_pressed(KeyCode::A) {
                picker.transfer_picker(PickerSelection::Circular);
            } else if is_key_pressed(KeyCode::S) {
                picker.transfer_picker(PickerSelection::Linear);
            }
        }
        self.picker.borrow().draw();

    }

    fn hit_detect(&mut self, pos: Vec2, node: &NodeStore) -> Vec<WeakNode> {
        if self.picker.borrow().bounding_box().contains(pos) {
            vec![node.get_weak()]
        } else {
            vec![]
        }
    }
}

impl New for PickerNode {
    type InType = ();
    type OutType = Rc<RefCell<Picker>>;
    fn new(_: Self::InType, _handler: &mut GenHandler) -> (Self::OutType, Self) {
        let rc = Rc::new(RefCell::new(Picker::new_circular(50.0, 10.0, [10.0, 270.0], 16.0, ColSelection::Hsva)));
        (
            rc.clone(),
            Self {
                picker: rc,
            }
        )
    }
}

impl PickerNode {
    // pub fn set_col(&mut self, col: Option<[f32; 4]>) { // Possibly dangerous, likely unneeded
    //     self.picker.borrow_mut().set_col(col);
    // }

    // pub fn transfer_col(&mut self, coltype: ColSelection) { // Possibly dangerous, likely unneeded
    //     self.picker.borrow_mut().transfer_col(coltype);
    // }

    pub fn transfer_picker(&mut self, pickertype: PickerSelection) {
        self.picker.borrow_mut().transfer_picker(pickertype);
    }

    pub fn get_col_rgba(&mut self) -> Option<[f32; 4]> {
        self.picker.borrow_mut().get_col_rgba()
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

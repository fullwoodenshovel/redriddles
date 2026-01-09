#![allow(dead_code)]
#![warn(unused_variables)]
#![warn(unused_imports)]

// TODO optimise colour picker by saving;

use std::rc::Rc;

use macroquad::prelude::*;

use redriddles::node::NodeStore;
use redriddles::ui::UserInputs;
use redriddles::ui::main::Main;


#[derive(PartialEq, Debug, Clone, Copy)]
enum Focus {
    Topbar,
    Sidebar,
    Picker,
    Draw,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum LastTouchFocus {
    None,
    SaveCol,
    TypeCol,
    Picker,
}

#[macroquad::main("Pixel Editor")]
async fn main() {
    let origin = NodeStore::origin::<Main>();
    let mut user_inputs = UserInputs::new(Rc::downgrade(&origin));
    loop {
        user_inputs.update();
        origin.borrow_mut().get_self_dyn_mut().update(&user_inputs);
    }
}
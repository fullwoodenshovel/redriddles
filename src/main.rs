use macroquad::prelude::*;

use redriddles::ui::Frame;
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
    let mut frame = Frame::new::<Main>();
    loop {
        frame.update().await
    }
}
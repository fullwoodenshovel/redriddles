#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use redriddles::ui::ResultFrame;
use redriddles::ui::main::Main;

#[macroquad::main("Pixel Editor")]
async fn main() {
    let mut frame = ResultFrame::new::<Main>();
    loop {
        frame.update().await
    }
}
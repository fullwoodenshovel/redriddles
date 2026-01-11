use redriddles::ui::Frame;
use redriddles::ui::main::Main;

#[macroquad::main("Pixel Editor")]
async fn main() {
    let mut frame = Frame::new::<Main>();
    loop {
        frame.update().await
    }
}
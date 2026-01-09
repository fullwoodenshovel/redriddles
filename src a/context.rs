use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};

async fn main() {
    let mut show_context_menu = false;
    let mut context_pos = vec2(0., 0.);

    loop {
        clear_background(DARKGRAY);

        // --- TOP BAR WITH SETTINGS BUTTON ---
        widgets::Window::new(hash!("top_bar"), vec2(0., 0.), vec2(screen_width(), 40.))
            .movable(false)
            .titlebar(false)
            .ui(&mut root_ui(), |ui| {
                ui.label(None, "Demo UI");
                ui.same_line(250.0);

                if ui.button(None, "Settings ⚙") {
                    // left-click on settings button
                    context_pos = mouse_position().into();
                    show_context_menu = true;
                }
            });

        let (mx, my) = mouse_position();

        // --- RIGHT CLICK ANYWHERE SHOWS CONTEXT MENU ---
        if is_mouse_button_pressed(MouseButton::Right) {
            show_context_menu = true;
            context_pos = vec2(mx, my);
        }

        // --- CONTEXT MENU (RIGHT-CLICK POPUP) ---
        if show_context_menu {
            let menu_w = 180.0;
            let menu_h = 150.0;

            widgets::Window::new(
                hash!("context_menu"),
                context_pos,
                vec2(menu_w, menu_h),
            )
            .titlebar(false)
            .movable(false)
            .ui(&mut root_ui(), |ui| {
                ui.label(None, "Settings");
                ui.separator();

                if ui.button(None, "Toggle Grid") {
                    // do something
                    show_context_menu = false;
                }
                if ui.button(None, "Export PNG") {
                    // do something
                    show_context_menu = false;
                }
                if ui.button(None, "Clear Canvas") {
                    // do something
                    show_context_menu = false;
                }
                ui.separator();
                if ui.button(None, "Close Menu") {
                    show_context_menu = false;
                }
            });

            // CLICK OUTSIDE MENU TO CLOSE
            if is_mouse_button_pressed(MouseButton::Left) {
                // If click is outside the popup rectangle
                if !(mx >= context_pos.x
                    && mx <= context_pos.x + menu_w
                    && my >= context_pos.y
                    && my <= context_pos.y + menu_h)
                {
                    show_context_menu = false;
                }
            }
        }

        next_frame().await;
    }
}

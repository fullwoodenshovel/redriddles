use std::path::PathBuf;

use super::*;

pub struct ExportSettings {
    pub path: Option<PathBuf>,
    pub process: ProcessSettings,
    pub place: PlaceSettings,
}

impl ExportSettings {
    pub fn new(path: Option<PathBuf>, temperature: f32, col_sel: ColSelection, pixel_size: u32, accept_transparent: f32) -> Self {
        Self {
            path,
            process: ProcessSettings { col_sel, pixel_size, accept_transparent, changed_this_frame: false },
            place: PlaceSettings { temperature, rect: None }
        }
    }
}

#[derive(Clone, Copy)]
pub struct ProcessSettings {
    pub col_sel: ColSelection,
    pub pixel_size: u32,
    pub accept_transparent: f32,
    pub changed_this_frame: bool
}

#[derive(Clone, Copy)]
pub struct PlaceSettings {
    pub temperature: f32,
    pub rect: Option<Rect>,
}

pub struct ExportSettingsNode {

}

impl New for ExportSettingsNode {
    fn new(_handler: &mut GenHandler) -> Self {
        Self {

        }
    }
}

impl Node for ExportSettingsNode {
    fn update(&mut self, ctx: &mut AppContextHandler, node: &NodeStore) {
        let process = &mut ctx.store.get_mut::<ExportSettings>().process;

        draw_text("Changing this requires textures to be reloaded.", 220.0, 130.0, 18.0, BLACK);
        
        if sub_ui_button(
            Rect::new(150.0, 150.0, 300.0, 38.0), &format!("Colour space averaging: {}", process.col_sel),
            DISABLEDCOL,
            DISABLEDHOVERCOL,
            node,
            ctx.user_inputs)
        {
            match process.col_sel {
                ColSelection::Hsva  => process.col_sel = ColSelection::OkLab,
                ColSelection::OkLab => process.col_sel = ColSelection::Rgba,
                ColSelection::Rgba  => process.col_sel = ColSelection::Hsva
            }
            process.changed_this_frame = true;
        }

        draw_text(&format!("Allowed transparency: {}", (process.accept_transparent * 255.0) as u8), 150.0, 220.0, 18.0, BLACK);
        for offset in 0..300 {
            let offset = offset as f32;
            let percent = offset / 300.0;
            let x = offset + 150.0;
            draw_rectangle(x, 230.0, 1.0, 18.0, Color { r: 0.0, g: 0.0, b: 0.0, a: percent });
        }
        draw_rectangle_lines(150.0, 230.0, 300.0, 18.0, 4.0, DISABLEDCOL);

        let current_x = process.accept_transparent * 300.0 + 150.0;
        draw_triangle(vec2(current_x, 250.0), vec2(current_x - 8.0, 258.0), vec2(current_x + 8.0, 258.0), BLACK);

        if ctx.user_inputs.left_mouse_down &&
            Rect::new(150.0, 230.0, 300.0, 18.0).contains(ctx.user_inputs.lasttouch_mouse) &&
            ctx.user_inputs.hoverhold_test(node)
        {
            process.accept_transparent = ((ctx.user_inputs.mouse.x - 150.0) / 300.0).clamp(0.0, 1.0);
            process.changed_this_frame = true;
        }

        if let Some(value) = slider(
            ENABLEDCOL,
            DISABLEDCOL,
            Rect::new(150.0, 280.0, 300.0, 18.0),
            &format!("Pixel size: {}", process.pixel_size),
            process.pixel_size as f32,
            0.0,
            256.0,
            ctx.user_inputs,
            node
        ) {
            process.pixel_size = value as u32;
            process.changed_this_frame = true;
        }
    }

    fn hit_detect(&mut self, pos: Vec2, node: &NodeStore, store: &mut Store) -> Vec<WeakNode> {
        node.hit_detect_children_and_self(pos, store)
    }
}
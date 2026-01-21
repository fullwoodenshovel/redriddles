use std::path::PathBuf;

use super::*;

pub struct ExportSettings {
    pub path: Option<PathBuf>,
    pub process: ProcessSettings,
    pub place: PlaceSettings,
}

impl ExportSettings {
    pub fn new(path: Option<PathBuf>, temperature: f32, col_sel: ColSelection, pixel_size: u32) -> Self {
        Self { path, process: ProcessSettings { col_sel, pixel_size }, place: PlaceSettings { temperature, rect: None } }
    }
}

#[derive(Clone, Copy)]
pub struct ProcessSettings {
    pub col_sel: ColSelection,
    pub pixel_size: u32
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
        disabled_ui_button(Rect::new(150.0, 150.0, 300.0, 38.0), "This does not yet exist", DISABLEDCOL)
    }

    fn hit_detect(&mut self, pos: Vec2, node: &NodeStore, store: &mut Store) -> Vec<WeakNode> {
        node.hit_detect_children_and_self(pos, store)
    }
}
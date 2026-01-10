use crate::ui::ColSelection;
use crate::ui::main::DrawState;
use bimap::BiMap;
use crate::helpers::*;
use macroquad::prelude::*;
use super::*;

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub enum ShortcutInstruction {
    None,
    ChangeDrawState(DrawState),
    Eraser,
    SaveCol,
    ToggleGrid,
    ChangePickerType(ColSelection),
}

pub struct Shortcuts {
    shortcuts: BiMap<Vec<KeyCode>, ShortcutInstruction>
}

impl Default for Shortcuts {
    fn default() -> Self {
        Self {
            shortcuts: vec![
                (vec![KeyCode::S], ShortcutInstruction::SaveCol),
                (vec![KeyCode::E], ShortcutInstruction::Eraser),
                (vec![KeyCode::H], ShortcutInstruction::ChangePickerType(ColSelection::Hsva)),
                (vec![KeyCode::R], ShortcutInstruction::ChangePickerType(ColSelection::Rgba)),
                (vec![KeyCode::O], ShortcutInstruction::ChangePickerType(ColSelection::OkLab)),
                (vec![KeyCode::F], ShortcutInstruction::ChangeDrawState(DrawState::Fill)),
                (vec![KeyCode::L], ShortcutInstruction::ChangeDrawState(DrawState::Line)),
                (vec![KeyCode::D], ShortcutInstruction::ChangeDrawState(DrawState::Draw)),
                (vec![KeyCode::P], ShortcutInstruction::ChangeDrawState(DrawState::Picker)),
                (vec![KeyCode::G], ShortcutInstruction::ToggleGrid),
            ].into_iter().map(|(mut key, value)| {
                Self::sort_key(&mut key);
                (key, value)
            }).collect()
        }
    }
}

impl Shortcuts {
    pub fn insert_no_overwrite(&mut self, mut key: Vec<KeyCode>, value: ShortcutInstruction) -> Result<(), (Vec<KeyCode>, ShortcutInstruction)> {
        Self::sort_key(&mut key);
        self.shortcuts.insert_no_overwrite(key, value)
    }

    pub fn insert(&mut self, mut key: Vec<KeyCode>, value: ShortcutInstruction) {
        Self::sort_key(&mut key);
        self.shortcuts.insert(key, value);
    }

    pub fn get_shortcuts(&self) -> Vec<(&Vec<KeyCode>, ShortcutInstruction)> {
        self.shortcuts.iter().map(|(k, &v)| (k, v)).collect()
    }

    pub fn get_output(&self, shortcut: &mut Vec<KeyCode>) -> ShortcutInstruction {
        Self::sort_key(shortcut);
        *self.shortcuts.get_by_left(shortcut).unwrap_or(&ShortcutInstruction::None)
    }

    pub fn get_shortcut(&self, output: &ShortcutInstruction) -> Option<&Vec<KeyCode>> {
        self.shortcuts.get_by_right(output)
    }

    fn sort_key(key: &mut [KeyCode]) {
        key.sort_by(|&a, &b| (a as u16).cmp(&(b as u16)));
    }
}

pub struct UserInputs {
    pub hoverhold_focus: Vec<WeakNode>,
    pub prev_hoverhold_focus: Vec<WeakNode>,
    pub hoverhold_mouse: Vec2,
    pub hover_focus: Vec<WeakNode>,
    pub prev_hover_focus: Vec<WeakNode>,
    pub lasttouch_focus: Vec<WeakNode>,
    pub lasttouch_mouse: Vec2,
    pub prev_lasttouch_focus: Vec<WeakNode>,
    pub prev_lasttouch_mouse: Vec2,
    pub mouse: Vec2,
    pub prev_mouse: Vec2,
    pub left_mouse_pressed: bool,
    pub right_mouse_pressed: bool,
    pub left_mouse_down: bool,
    pub right_mouse_down: bool,
    pub left_let_go: bool,
    pub right_let_go: bool,
    pub pressed_shortcut: ShortcutInstruction,
    pub held_shortcut: ShortcutInstruction,
    pub shortcuts: Shortcuts,
    origin: StrongNode,
}


impl UserInputs {
    pub fn new(origin: &Origin) -> Self {
        Self {
            hoverhold_focus: vec![],
            prev_hoverhold_focus: vec![],
            hoverhold_mouse: Vec2::splat(0.0),
            hover_focus: vec![],
            prev_hover_focus: vec![],
            lasttouch_focus: vec![],
            lasttouch_mouse: Vec2::splat(0.0),
            prev_lasttouch_focus: vec![],
            prev_lasttouch_mouse: Vec2::splat(0.0),
            mouse: Vec2::splat(0.0),
            prev_mouse: Vec2::splat(0.0),
            left_mouse_pressed: false,
            right_mouse_pressed: false,
            left_mouse_down: false,
            right_mouse_down: false,
            left_let_go: false,
            right_let_go: false,
            pressed_shortcut: ShortcutInstruction::None,
            held_shortcut: ShortcutInstruction::None,
            shortcuts: Shortcuts::default(),
            origin: origin.node.clone(),
        }
    }

    pub fn update(&mut self, store: &mut Store) {
        self.prev_mouse = self.mouse;
        self.mouse = mouse_vec();
        self.left_mouse_pressed = is_mouse_button_pressed(MouseButton::Left);
        self.right_mouse_pressed = is_mouse_button_pressed(MouseButton::Right);
        self.left_mouse_down = is_mouse_button_down(MouseButton::Left);
        self.right_mouse_down = is_mouse_button_down(MouseButton::Right);
        self.left_let_go = is_mouse_button_released(MouseButton::Left);
        self.right_let_go = is_mouse_button_released(MouseButton::Right);

        self.prev_hover_focus = std::mem::take(&mut self.hover_focus);
        self.hover_focus = Handler::new(&self.origin).hit_detect(self.mouse, store);

        let new = self.shortcuts.get_output(&mut get_keys_down().into_iter().collect());
        if new != self.held_shortcut {
            self.pressed_shortcut = new;
        } else {
            self.pressed_shortcut = ShortcutInstruction::None;
        }
        self.held_shortcut = new;

        if self.left_mouse_pressed || !self.left_mouse_down {
            self.prev_hoverhold_focus = std::mem::take(&mut self.hoverhold_focus);
            self.hoverhold_focus = self.hover_focus.clone();
            self.hoverhold_mouse = self.mouse;
            if self.left_mouse_pressed {
                self.prev_lasttouch_focus = std::mem::take(&mut self.lasttouch_focus);
                self.prev_lasttouch_mouse = self.lasttouch_mouse;
                self.lasttouch_focus = self.hover_focus.clone();
                self.lasttouch_mouse = self.mouse;
            }
        }
    }

    pub fn hoverhold_test(&self, node: &NodeStore) -> bool {
        node.contains_self(&self.hoverhold_focus)
    }

    pub fn prev_hoverhold_test(&self, node: &NodeStore) -> bool {
        node.contains_self(&self.prev_hoverhold_focus)
    }

    pub fn hover_test(&self, node: &NodeStore) -> bool {
        node.contains_self(&self.hover_focus)
    }

    pub fn prev_hover_test(&self, node: &NodeStore) -> bool {
        node.contains_self(&self.prev_hover_focus)
    }

    pub fn last_touch_test(&self, node: &NodeStore) -> bool {
        node.contains_self(&self.lasttouch_focus)
    }

    pub fn prev_last_touch_test(&self, node: &NodeStore) -> bool {
        node.contains_self(&self.prev_lasttouch_focus)
    }

    pub fn instruction_pressed(&self, instruction: ShortcutInstruction) -> bool {
        self.pressed_shortcut == instruction
    }

    pub fn instruction_active(&self, instruction: ShortcutInstruction) -> bool {
        self.held_shortcut == instruction
    }
}
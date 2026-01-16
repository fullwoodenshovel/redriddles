use std::fmt::{Display, Error};
use std::str::FromStr;

use crate::node::expanded_keycode::ExKeyCode;
use crate::ui::ColSelection;
use crate::ui::main::{DrawState, Tab};
use bimap::BiMap;
use serde::de::Visitor;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use crate::helpers::*;
use macroquad::prelude::*;
use super::*;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum ShortcutInstruction { // If changing this, update Deserialize, ORDER, FromStr
    None,
    ChangeDrawState(DrawState),
    Eraser,
    SaveCol,
    ToggleGrid,
    ChangePickerType(ColSelection),
    GoTo(Tab)
}

impl Serialize for ShortcutInstruction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(&format!("{self}"))
    }
}

impl<'de> Deserialize<'de> for ShortcutInstruction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        
        struct ShortcutVisitor;
        impl<'de> Visitor<'de> for ShortcutVisitor {
            type Value = ShortcutInstruction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a string representing a shortcut instruction")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where E: serde::de::Error, {
                match FromStr::from_str(v) {
                    Ok(result) => Ok(result),
                    Err(_) => Err(E::custom(format!("Value `{v}` is invalid.")))
                }
            }
        }

        deserializer.deserialize_str(ShortcutVisitor)
    }
}

const ORDER: [ShortcutInstruction; 13] = [
    ShortcutInstruction::ChangeDrawState(DrawState::Draw),
    ShortcutInstruction::ChangeDrawState(DrawState::Fill),
    ShortcutInstruction::ChangeDrawState(DrawState::Line),
    ShortcutInstruction::ChangeDrawState(DrawState::Picker),
    ShortcutInstruction::Eraser,
    ShortcutInstruction::SaveCol,
    ShortcutInstruction::ToggleGrid,
    ShortcutInstruction::ChangePickerType(ColSelection::Hsva),
    ShortcutInstruction::ChangePickerType(ColSelection::Rgba),
    ShortcutInstruction::ChangePickerType(ColSelection::OkLab),
    ShortcutInstruction::GoTo(Tab::Draw),
    ShortcutInstruction::GoTo(Tab::Settings),
    ShortcutInstruction::GoTo(Tab::Export),
];

impl Display for ShortcutInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => Err(Error),
            Self::ChangeDrawState(draw_state) => write!(f, "Change draw state to {}", draw_state),
            Self::Eraser => write!(f, "Eraser"),
            Self::SaveCol => write!(f, "Save colour"),
            Self::ToggleGrid => write!(f, "Toggle grid"),
            Self::ChangePickerType(col_type) => write!(f, "Change picker colour space to {}", col_type),
            Self::GoTo(tab) => write!(f, "Go to tab {}", tab),
        }
    }
}

impl FromStr for ShortcutInstruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match s {
            "None" => Self::None,
            "Eraser" => Self::Eraser,
            "Save colour" | "Save color" | "Save col" => Self::SaveCol,
            "Toggle grid" => Self::ToggleGrid,
            
            s if s.starts_with("Change draw state to ") => Self::ChangeDrawState(DrawState::from_str(&s[21..])?),
            s if s.starts_with("Change picker colour space to ") => Self::ChangePickerType(ColSelection::from_str(&s[30..])?),
            s if s.starts_with("Go to tab ") => Self::GoTo(Tab::from_str(&s[10..])?),

            _ => return Err(())
        };
        Ok(result)
    }
}

pub struct Shortcuts {
    pub(super) shortcuts: BiMap<Vec<KeyCode>, ShortcutInstruction>,
    pub(super) empty: Vec<KeyCode>
}

pub fn shortcut_to_string(shortcut: &[KeyCode]) -> String {
    shortcut.iter().map(|d| prettify_camel_case(format!("{d:?}"))).collect::<Vec<_>>().join(" + ")
}

pub fn prettify_camel_case(str: String) -> String {
    let mut result = String::with_capacity(str.len() + str.matches(char::is_uppercase).count());
    let mut chars = str.chars();

    if let Some(char) = chars.next() {
        result.push(char);
    }

    for char in chars {
        if char.is_uppercase() {
            result.push(' ');
        }
        result.push(char);
    }

    result
}

pub fn string_to_shortcut(shortcut: &str) -> Result<Vec<KeyCode>, String> {
    let mut result = Vec::new();
    for string in shortcut.split('+').map(|s| s.trim()) {
        let string = string.replace(' ', "");
        match from_str::<ExKeyCode>(&format!("\"{string}\"")) {
            Ok(item) => result.push(item.into()),
            Err(_) => return Err(format!("`{string}` is not a valid key.\n\
                If you want to see how to specify a specific key,\n\
                create a keyboard shortcut with it and see what pops up."))
        }
    }
    Ok(result)
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
                (vec![KeyCode::LeftControl, KeyCode::D], ShortcutInstruction::GoTo(Tab::Draw)),
                (vec![KeyCode::LeftControl, KeyCode::S], ShortcutInstruction::GoTo(Tab::Settings)),
                (vec![KeyCode::LeftControl, KeyCode::E], ShortcutInstruction::GoTo(Tab::Export)),
            ].into_iter().collect(),
            empty: Vec::new()
        }
    }
}

impl Shortcuts {
    pub fn insert_no_overwrite(&mut self, key: Vec<KeyCode>, value: ShortcutInstruction) -> Result<(), (Vec<KeyCode>, ShortcutInstruction)> {
        self.shortcuts.insert_no_overwrite(key, value)
    }

    pub fn insert(&mut self, shortcut: Vec<KeyCode>, instruction: ShortcutInstruction) {
        self.shortcuts.insert(shortcut, instruction);
    }

    pub fn discard(&mut self, instruction: ShortcutInstruction) {
        self.shortcuts.remove_by_right(&instruction);
    }

    pub fn get_shortcuts(&self) -> impl Iterator<Item = (&Vec<KeyCode>, ShortcutInstruction)> {
        ORDER.iter().map(|&d| match self.shortcuts.get_by_right(&d) {
                Some(shortcut) => (shortcut, d),
                None => (&self.empty, d)
            }
        )
    }

    pub fn get_owned_shortcuts(&self) -> Vec<(Vec<KeyCode>, ShortcutInstruction)> {
        ORDER.iter().map(|&d| match self.shortcuts.get_by_right(&d) {
                Some(shortcut) => (shortcut.clone(), d),
                None => (Vec::new(), d)
            }
        ).collect()
    }

    pub fn get_output(&self, shortcut: &Vec<KeyCode>) -> ShortcutInstruction {
        if shortcut.is_empty() {
            ShortcutInstruction::None
        } else {
            *self.shortcuts.get_by_left(shortcut).unwrap_or(&ShortcutInstruction::None)
        }
    }

    pub fn get_shortcut(&self, output: &ShortcutInstruction) -> Option<&Vec<KeyCode>> {
        self.shortcuts.get_by_right(output)
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
    pub pressed_instruction: ShortcutInstruction,
    pub held_instruction: ShortcutInstruction,
    pub held_keyboard: Vec<KeyCode>,
    pub prev_held_keyboard: Vec<KeyCode>,
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
            pressed_instruction: ShortcutInstruction::None,
            held_instruction: ShortcutInstruction::None,
            held_keyboard: vec![],
            prev_held_keyboard: vec![],
            origin: origin.node.clone(),
        }
    }

    pub fn update(&mut self, store: &mut Store, shortcuts: &Shortcuts) {
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

        self.prev_held_keyboard = self.held_keyboard.clone();
        self.held_keyboard.retain(|&k| is_key_down(k));
        self.held_keyboard.append(&mut get_keys_pressed().into_iter().collect());

        let new = shortcuts.get_output(&self.held_keyboard);
        if new != self.held_instruction {
            self.pressed_instruction = new;
        } else {
            self.pressed_instruction = ShortcutInstruction::None;
        }
        self.held_instruction = new;

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
        self.pressed_instruction == instruction
    }

    pub fn instruction_active(&self, instruction: ShortcutInstruction) -> bool {
        self.held_instruction == instruction
    }
}
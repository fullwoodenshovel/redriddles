use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum DrawState {
    Line,
    Fill,
    Picker,
    Draw,
}

impl Display for DrawState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Line => write!(f, "Line"),
            Self::Fill => write!(f, "Fill"),
            Self::Picker => write!(f, "Picker"),
            Self::Draw => write!(f, "Draw"),
        }
    }
}

impl FromStr for DrawState {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match s {
            "Line" => Self::Line,
            "Fill" => Self::Fill,
            "Picker" => Self::Picker,
            "Draw" => Self::Draw,
            _ => return Err(())
        };
        Ok(result)
    }
}

pub struct DrawStateButton {
    name: &'static str,
    pub rect: Rect,
    new_state: DrawState
}

impl NewNoOut for DrawStateButton {
    type InType = (&'static str, Rect, DrawState);
    fn new((name, rect, new_state): Self::InType, _handler: &mut GenHandler) -> Self {
        Self {
            name,
            rect,
            new_state
        }
    }
}

impl Node for DrawStateButton {
    fn update(&mut self, ctx: &mut AppContextHandler, node: &NodeStore) {
        let hovered = ctx.user_inputs.hover_test(node);
        let lasttouch = ctx.user_inputs.last_touch_test(node);
        let active = *ctx.store.get::<DrawState>() == self.new_state;

        raw_ui_button(
            self.rect,
            self.name,
            hovered,
            false,
            if active { ENABLEDCOL } else { DISABLEDCOL },
            if active { ENABLEDHOVERCOL } else { DISABLEDHOVERCOL }
        );

        if lasttouch && hovered && ctx.user_inputs.left_let_go {
            if active {
                ctx.store.overwrite(DrawState::Draw);
            } else {
                ctx.store.overwrite(self.new_state);
            }
        }
    }

    fn hit_detect(&mut self, pos: Vec2, node: &NodeStore, _store: &mut Store) -> Vec<WeakNode> {
        if self.rect.contains(pos) {
            vec![node.get_weak()]
        } else {
            vec![]
        }
    }
}
use std::{fmt::Display, str::FromStr};

use super::*;
mod topbar;
mod draw;
mod settings;
mod export;
use serde::{Deserialize, Serialize};
use topbar::{Topbar, status};
use draw::Draw;
use settings::Settings;
use export::Export;
pub use draw::DrawState;

pub struct Main {
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub enum Tab {
    Draw,
    Settings,
    Export
}

impl Display for Tab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Draw => write!(f, "Draw"),
            Self::Settings => write!(f, "Settings"),
            Self::Export => write!(f, "Export"),
        }
    }
}

impl FromStr for Tab {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match s {
            "Draw" => Self::Draw,
            "Settings" => Self::Settings,
            "Export" => Self::Export,
            _ => return Err(())
        };
        Ok(result)
    }
}

const TOPBAR: usize = 3;
impl New for Main { // 0 is draw, 1 is settings, 2 is export, 3 is topbar
    fn new(handler: &mut GenHandler) -> Self {
        handler.push_child::<Draw>();
        handler.push_child::<Settings>();
        handler.push_child::<Export>();

        status::push::<0>(handler);
        handler.push_child_io::<Topbar<0>>((
            156.0,
            "Pixel Editor",
            Box::new([
                "Draw",
                "Settings",
                "Export",
            ])
        ));

        Self {}
    }
}

impl Node for Main {
    fn update(&mut self, ctx: &mut AppContextHandler, node: &NodeStore) {
        if let ShortcutInstruction::GoTo(tab) = ctx.user_inputs.pressed_instruction {
            status::set::<0>(ctx.store, tab as u8);
        }

        let children = node.get_children();
        let status = status::get_or_default::<0>(ctx.store);
        children[status as usize].update(ctx);
        children[TOPBAR].update(ctx);
    }

    fn hit_detect(&mut self, pos: Vec2, node: &NodeStore, store: &mut Store) -> Vec<WeakNode> {
        let children = node.get_children();
        let mut result = children[TOPBAR].hit_detect(pos, store);
        if result.is_empty() {
            result = node.get_children()[status::get_or_default::<0>(store) as usize].hit_detect(pos, store);
        }
        result.push(node.get_weak());
        result
    }
}
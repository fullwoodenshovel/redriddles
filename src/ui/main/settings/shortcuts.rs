use super::*;

#[derive(PartialEq, Eq)]
enum OverwriteState {
    Active(usize),
    Disabled(usize),
    New
}

pub struct Shortcuts {
    active: OverwriteState
}

impl New for Shortcuts {
    fn new(_handler: &mut GenHandler) -> Self {
        Self {
            active: OverwriteState::New
        }
    }
}

impl Node for Shortcuts {
    fn update(&mut self, ctx: &mut AppContextHandler, node: &NodeStore) {
        let hover_possible = ctx.user_inputs.hoverhold_test(node);
        for (index, (shortcut, instruction)) in ctx.save_data.shortcuts.get_owned_shortcuts().into_iter().enumerate() {
            disabled_ui_button(Rect::new(26.0, 32.0 * index as f32 + 106.0, 300.0, 26.0), &format!("{instruction}"), DISABLEDCOL);
            
            let remove_rect = Rect::new(352.0, 32.0 * index as f32 + 106.0, 80.0, 26.0);
            let shortcut_rect = Rect::new(438.0, 32.0 * index as f32 + 106.0, 300.0, 26.0);
            let name;
            if shortcut_rect.contains(ctx.user_inputs.lasttouch_mouse) && self.active != OverwriteState::Disabled(index) {
                name = shortcut_to_string(&ctx.user_inputs.prev_held_keyboard);
                self.active = OverwriteState::Active(index);
                disabled_ui_button(remove_rect, "Remove", DISABLEDCOL);
            } else if shortcut.is_empty() {
                disabled_ui_button(remove_rect, "Removed", ENABLEDCOL);
                name = "Click to add.".to_string();
            } else if ui_button(remove_rect, "Remove", DISABLEDCOL, DISABLEDHOVERCOL, node, ctx) {
                ctx.save_data.shortcuts.discard(instruction);
                name = "Click to add.".to_string();
            } else {
                name = shortcut_to_string(&shortcut);
            }

            raw_ui_button(
                shortcut_rect,
                &name,
                hover_possible && shortcut_rect.contains(ctx.user_inputs.mouse),
                false,
                if self.active == OverwriteState::Active(index) {ENABLEDCOL} else {DISABLEDCOL},
                if self.active == OverwriteState::Active(index) {ENABLEDCOL} else {DISABLEDHOVERCOL}
            );

            if self.active == OverwriteState::Active(index) {
                if !shortcut_rect.contains(ctx.user_inputs.lasttouch_mouse) {
                    self.active = OverwriteState::New;
                } else if ctx.user_inputs.prev_held_keyboard.len() > ctx.user_inputs.held_keyboard.len() {
                    let shortcut = ctx.user_inputs.prev_held_keyboard.clone();
                    ctx.save_data.shortcuts.insert(shortcut, instruction);
                    self.active = OverwriteState::Disabled(index);
                }
            }
        }
    }
    
    fn hit_detect(&mut self, pos: Vec2, node: &NodeStore, store: &mut Store) -> Vec<WeakNode> {
        node.hit_detect_children_and_self(pos, store)
    }
}
use super::*;
use save::{LastTouch, PrevTouch};

pub struct SaveGrid {
    rect: Rect,
    saved_cols: Vec<[Option<[f32; 4]>; 4]>,
}

impl New for SaveGrid {
    fn new(_handler: &mut GenHandler) -> Self {
        Self {
            rect: Rect::new(10.0, 60.0, 132.0, 198.0),
            saved_cols: vec![[None; 4]; 6],
        }
    }
}

impl Node for SaveGrid {
    fn update(&mut self, ctx: &mut AppContextHandler, node: &NodeStore) {
        let hovered = ctx.user_inputs.hover_test(node);
        let lasttouch = ctx.store.value::<LastTouch>();
        let prevlasttouch = ctx.store.value::<PrevTouch>();

        for (y, cols) in self.saved_cols.iter_mut().enumerate() {
            for (x, col) in cols.iter_mut().enumerate() {
                let x = 33.0 * x as f32 + 10.0;
                let y = 33.0 * y as f32 + 60.0;
                let rect = Rect::new(x, y, 28.0, 28.0);

                if col_button(
                    rect,
                    hovered && rect.contains(ctx.user_inputs.mouse),
                    ctx.user_inputs.left_let_go,
                    if lasttouch { ENABLEDCOL } else { DISABLEDCOL },
                    if lasttouch { ENABLEDHOVERCOL } else { DISABLEDHOVERCOL }
                ) {
                    if prevlasttouch {
                        if let Some(new) = ctx.store.get_mut::<Picker>().get_col_rgba() {
                            *col = Some(new);
                        }
                    } else {
                        ctx.store.get_mut::<Picker>().set_col(*col);
                    }
                }

                if let Some(col) = col {
                    draw_rectangle(x + 4.0, y + 4.0, 20.0, 20.0, arr_to_macroquad(*col));
                }
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
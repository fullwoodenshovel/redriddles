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
    fn update(&mut self, ctx: &mut AppContextHandler, _node: &NodeStore) {
        let hover_possible = ctx.store.value::<HoverPossible>();
        let lasttouch = ctx.store.value::<LastTouch>();
        let prevlasttouch = ctx.store.value::<PrevTouch>();

        for (y, cols) in self.saved_cols.iter_mut().enumerate() {
            for (x, col) in cols.iter_mut().enumerate() {
                let x = 33.0 * x as f32 + 10.0;
                let y = 33.0 * y as f32 + 60.0;

                if col_button(
                    Rect::new(x, y, 28.0, 28.0),
                    if hover_possible {Some(ctx.user_inputs.mouse)} else {None},
                    ctx.user_inputs.left_let_go,
                    if lasttouch { Color { r: 0.65, g: 0.8, b: 0.65, a: 1.0 } } else { LIGHTGRAY },
                    if lasttouch { Color { r: 0.60, g: 0.75, b: 0.60, a: 1.0 } } else { Color { r: 0.65, g: 0.65, b: 0.65, a: 1.0 } }
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
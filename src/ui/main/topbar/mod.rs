use super::*;
mod change_status;
use change_status::ChangeStatus;

pub struct Topbar<const INDEX: u8> {
    size: Vec2,
    label: &'static str
}

#[tuple_deref]
struct Status<const INDEX: u8>(pub Option<u8>);

pub mod status {
    use super::*;

    // pub fn get<const INDEX: u8>(store: &mut Store) -> Option<u8> {
    //     store.value::<Status<INDEX>>()
    // }

    pub fn get_or_default<const INDEX: u8>(store: &mut Store) -> u8 {
        store.unwrap_or_set_default::<Status<INDEX>>()
    }

    pub fn set<const INDEX: u8>(store: &mut Store, value: u8) {
        store.set::<Status<INDEX>>(Some(value));
        if INDEX == 0 {
            store.set::<Status<1>>(None);
        }
        // if INDEX <= 1 { // Copy this down when adding lower level statuses
        //     store.set::<Status<1>>(None);
        // }
    }

    pub fn push<const INDEX: u8>(handler: &mut GenHandler) {
        if INDEX > 1 {
            panic!("Status<{}> attempted to get pushed", INDEX)
        }
        handler.push_data(Status::<INDEX>(None));
    }

    pub fn push_nocheck<const INDEX: u8>(handler: &mut GenHandler) {
        if INDEX > 1 {
            panic!("Status<{}> attempted to get pushed", INDEX)
        }
        handler.push_data_nocheck(Status::<INDEX>(None));
    }
}

impl<const INDEX: u8> NewNoOut for Topbar<INDEX> {
    type InType = (f32, &'static str, Box<[&'static str]>);
    fn new((mut x_offset, label, names): Self::InType, handler: &mut GenHandler) -> Self {
        for (index, name) in names.iter().enumerate() {
            x_offset += 6.0 + handler.push_child_io::<ChangeStatus<INDEX>>((name.to_string(), x_offset, index as u8));
        }

        Self {
            size: vec2(screen_width(), 40.0),
            label
        }
    }
}

impl<const INDEX: u8> Node for Topbar<INDEX> {
    fn update(&mut self, ctx: &mut AppContextHandler, node: &NodeStore) {
        self.size.x = screen_width();
        let starty = INDEX as f32 * 42.0;
        if INDEX != 0 {
            draw_rectangle(0.0, starty - 2.0, self.size.x, 2.0, BLACK);
        }
        draw_rectangle(0.0, starty, self.size.x, self.size.y, DARKGRAY);

        draw_text(self.label, 10.0, starty + 26.0, 22.0, WHITE);
        node.update_children(ctx);
    }

    fn hit_detect(&mut self, pos: Vec2, node: &NodeStore, store: &mut Store) -> Vec<WeakNode> {
        if self.bounding_box().contains(pos) {
            node.hit_detect_children_and_self(pos, store)
        } else {
            vec![]
        }
    }
}

impl<const INDEX: u8> Topbar<INDEX> {
    fn bounding_box(&self) -> Rect {
        Rect::new(0.0, INDEX as f32 * 42.0, self.size.x, self.size.y)
    }
}
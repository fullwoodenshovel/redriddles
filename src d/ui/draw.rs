use super::*;
use topbar::Topbar;
use sidebar::Sidebar;
use draw_section::DrawSection;

pub struct Draw;

impl NewNoOut for Draw {
    type InType = ();
    fn new(_: Self::InType, handler: &mut GenHandler) -> Self { // 0 is Topbar, 1 is Sidebar, 2 is DrawSection
        handler.push_child::<Topbar>(());
        let picker = handler.push_child::<Sidebar>(());
        handler.push_child::<DrawSection>(picker);

        Self
    }
}

impl Node for Draw {
    fn update(&mut self, user_inputs: &UserInputs, node: &NodeStore) {
        clear_background(WHITE);
        let mut children = node.get_children();
        let (Some(draw_section), Some(sidebar), Some(topbar)) = (children.pop(), children.pop(), children.pop()) else { panic!("Children malformed in Main.") };
        draw_section.update(user_inputs);
        topbar.update(user_inputs);
        sidebar.update(user_inputs);
    }

    fn hit_detect(&mut self, pos: Vec2, node: &NodeStore) -> Vec<WeakNode> {
        node.hit_detect_children_and_self(pos)
    }
}
// use super::*;

// pub struct Example {

// }

// impl NewNoOut for Example {
//     type InType = ();
//     fn new(_: Self::InType, _handler: &mut GenHandler) -> Self {
//         Self {

//         }
//     }
// }

// impl Node for Example {
//     fn update(&mut self, ctx: &mut AppContextHandler, node: &NodeStore) {

//     }

//     fn hit_detect(&mut self, pos: Vec2, node: &NodeStore, store: &mut Store) -> Vec<WeakNode> {
//         if self.rect.contains(pos) {
//             vec![node.get_weak()]
//         } else {
//             vec![]
//         }
//     }
// }
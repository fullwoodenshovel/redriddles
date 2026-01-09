#[cfg(feature = "debug")]
use super::debug::RefCell;
#[cfg(not(feature = "debug"))]
use std::cell::RefCell;
use std::any::Any;
use std::ops::Deref;
use std::rc::{Rc, Weak};

use macroquad::prelude::*;

use super::helpers::*;


pub struct UserInputs {
    pub hoverhold_focus: Vec<WeakNode>,
    pub hoverhold_mouse: Vec2,
    pub hover_focus: Vec<WeakNode>,
    pub lasttouch_focus: Vec<WeakNode>,
    pub lasttouch_mouse: Vec2,
    pub mouse: Vec2,
    pub left_mouse_pressed: bool,
    pub right_mouse_pressed: bool,
    pub left_mouse_down: bool,
    pub right_mouse_down: bool,
    pub left_let_go: bool,
    pub right_let_go: bool,
    origin: StrongNode,
}

impl UserInputs {
    pub fn new(origin: &Origin) -> Self {
        Self {
            hoverhold_focus: vec![],
            hoverhold_mouse: Vec2::splat(0.0),
            hover_focus: vec![],
            lasttouch_focus: vec![],
            lasttouch_mouse: Vec2::splat(0.0),
            mouse: Vec2::splat(0.0),
            left_mouse_pressed: false,
            right_mouse_pressed: false,
            left_mouse_down: false,
            right_mouse_down: false,
            left_let_go: false,
            right_let_go: false,
            origin: origin.node.clone(), 
        }
    }

    pub fn update(&mut self) {
        self.mouse = mouse_vec();
        self.left_mouse_pressed = is_mouse_button_pressed(MouseButton::Left);
        self.right_mouse_pressed = is_mouse_button_pressed(MouseButton::Right);
        self.left_mouse_down = is_mouse_button_down(MouseButton::Left);
        self.right_mouse_down = is_mouse_button_down(MouseButton::Right);
        self.left_let_go = is_mouse_button_released(MouseButton::Left);
        self.right_let_go = is_mouse_button_released(MouseButton::Right);
        

        self.hover_focus = Handler::new(&self.origin).hit_detect(self.mouse);

        if self.left_mouse_pressed || !self.left_mouse_down {
            self.hoverhold_focus = self.hover_focus.clone();
            self.hoverhold_mouse = self.mouse;
            if self.left_mouse_pressed {
                self.lasttouch_focus = self.hover_focus.clone();
                self.lasttouch_mouse = self.mouse;
            }
        }
    }

    pub fn hoverhold_test(&self, node: &NodeStore) -> bool {
        node.contains_self(&self.hoverhold_focus)
    }

    pub fn hover_test(&self, node: &NodeStore) -> bool {
        node.contains_self(&self.hover_focus)
    }

    pub fn last_touch_test(&self, node: &NodeStore) -> bool {
        node.contains_self(&self.lasttouch_focus)
    }
}


pub fn weak_from_opt<T: Node + 'static>(this: &Option<WeakRef<T>>) -> WeakRef<dyn Node> {
    let result = this.clone().unwrap();
    result as WeakRef<dyn Node>
}

pub fn weak_from<T: Node + 'static>(this: &WeakRef<T>) -> WeakRef<dyn Node> {
    let result = this.clone();
    result as WeakRef<dyn Node>
}

pub fn contains_node(vec: &Vec<WeakRef<dyn Node>>, test: &WeakRef<dyn Node>) -> bool {
    let Some(upgraded) = &test.upgrade() else { return false; };
    vec.iter().any(|d|
        if let Some(a) = d.upgrade() {
            Rc::ptr_eq(&a, upgraded)
        } else { false }
    )
}

pub type StrongRef<T> = Rc<RefCell<T>>;
pub type WeakRef<T> = Weak<RefCell<T>>;
pub type WeakNode = Weak<NodeStore>;
pub type StrongNode = Rc<NodeStore>;

#[allow(private_bounds)]
pub trait Node: AsAny {
    fn update(&mut self, user_inputs: &UserInputs, node: &NodeStore);
    fn hit_detect(&mut self, pos: Vec2, node: &NodeStore) -> Vec<WeakNode>;
}

trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[allow(private_bounds)]
pub trait AsAnyExt: AsAny {
    fn try_get_self_mut<T: Node + 'static>(&mut self) -> Option<&mut T>;
    fn try_get_self<T: Node + 'static>(&self) -> Option<&T>;
    fn get_self<T: Node + 'static>(&self) -> &T;
    fn get_self_mut<T: Node + 'static>(&mut self) -> &mut T;
}

impl<A: Any> AsAny for A {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl<A: AsAny> AsAnyExt for A {
    fn try_get_self<T: Node + 'static>(&self) -> Option<&T> {
        self.as_any().downcast_ref()
    }
    
    fn try_get_self_mut<T: Node + 'static>(&mut self) -> Option<&mut T> {
        self.as_any_mut().downcast_mut()
    }
    
    fn get_self<T: Node + 'static>(&self) -> &T {
        self.try_get_self().unwrap_or_else(|| panic!("In `get_self`, {DOWNCAST_FAILED}"))
    }
    
    fn get_self_mut<T: Node + 'static>(&mut self) -> &mut T {
        self.try_get_self_mut().unwrap_or_else(|| panic!("In `get_self_mut`, {DOWNCAST_FAILED}"))
    }
}

pub trait New {
    type InType;
    type OutType;
    fn new(data: Self::InType, handler: &mut GenHandler) -> (Self::OutType, Self);
}

pub trait NewNoOut {
    type InType;
    fn new(data: Self::InType, handler: &mut GenHandler) -> Self;
}

impl<T: NewNoOut> New for T {
    type InType = T::InType;
    type OutType = ();
    fn new(data: Self::InType, handler: &mut GenHandler) -> (Self::OutType, Self) {
        ((), T::new(data, handler))
    }
}

pub struct GenHandler {
    children: Vec<Rc<NodeStore>>
}

pub struct Handler<'a> {
    this: &'a NodeStore,
}

impl<'a> Handler<'a> {
    fn new(this: &'a NodeStore) -> Self {
        Handler {
            this,
        }
    }
}

impl<'a> Handler<'a> {
    pub fn hit_detect(&self, pos: Vec2) -> Vec<WeakNode> {
        self.this.node.borrow_mut().hit_detect(pos, self.this)
    }

    pub fn update(&self, user_inputs: &UserInputs) {
        self.this.node.borrow_mut().update(user_inputs, self.this);
    }

    pub fn get_self_dyn<T, F: FnOnce(&dyn Node) -> T>(&self, f: F) -> T {
        f(self.this.node.borrow().as_ref())
    }
    
    pub fn get_self_dyn_mut<T, F: FnOnce(&mut dyn Node) -> T>(&self, f: F) -> T {
        f(self.this.node.borrow_mut().as_mut())
    }
}

impl<'a> Deref for Handler<'a> {
    type Target = NodeStore;
    fn deref(&self) -> &Self::Target {
        self.this
    }
}

pub struct NodeStore {
    this: WeakNode,
    node: RefCell<Box<dyn Node>>,
    children: Vec<StrongNode>
}

pub struct Origin {
    node: StrongNode
}

impl Origin {
    pub fn get_handler(&'_ self) -> Handler<'_> {
        Handler::new(&self.node)
    }
}

const SELF_THIS_INVALID: &str = "pointer to self dropped and upgrade impossible. This means `self.this` did not point to self.";
const INVALID_WEAK: &str = "pointer dropped and upgrade impossible.";
const DOWNCAST_FAILED: &str = "downcast failed. Either tree is malformed, or an incorrect type was supplied.";

impl GenHandler {
    pub fn push_child<Child: Node + New + 'static>(&mut self, data: Child::InType) -> Child::OutType {
        let (out, node) = NodeStore::new::<Child>(data);
        self.children.push(
            node
        );
        out
    }
}

impl NodeStore {
    pub fn origin<T: Node + New<InType = (), OutType = ()> + 'static>() -> Origin {
        Origin {node: Self::new::<T>(()).1 }
    }
    
    fn new<Child: Node + New + 'static>(data: Child::InType) -> (Child::OutType, StrongNode) {
        let mut handler = GenHandler { children: vec![] };
        let (out, node) = Child::new(data, &mut handler);
        let node = RefCell::new(Box::new(node));
        (
            out,
            Rc::new_cyclic(|me| {
                Self {
                    this: me.clone(),
                    children: handler.children,
                    node
                }
            })
        )
    }
    
    pub fn push_child<Child: Node + New + 'static>(&mut self, data: Child::InType) -> Child::OutType {
        let (out, node) = NodeStore::new::<Child>(data);
        self.children.push(
            node
        );
        out
    }
    
    pub fn get_children(&'_ self) -> Vec<Handler<'_>>{
        self.children.iter().map(|c| Handler::new(c.as_ref())).collect()
    }

    pub fn update_children(&self, user_inputs: &UserInputs) {
        for child in self.get_children() {
            child.update(user_inputs)
        }
    }

    pub fn hit_detect_children_and_self(&self, pos: Vec2) -> Vec<WeakNode>{
        let mut result = vec![];
        for child in self.get_children() {
            result = child.hit_detect(pos);
            if !result.is_empty() {
                break
            }
        }
        result.push(self.get_weak());
        result
    }
    
    fn contains_self(&self, vec: &[WeakNode]) -> bool {
        let this = self.this.clone().upgrade().unwrap_or_else(|| panic!("In `contains_self`, {SELF_THIS_INVALID}"));
        vec.iter().any(|d|
            Rc::ptr_eq(&this, &d.upgrade().unwrap_or_else(|| panic!("In `contains_self`, {INVALID_WEAK}")))
        )
    }

    pub fn get_weak(&self) -> WeakNode {
        self.this.clone()
    }
}
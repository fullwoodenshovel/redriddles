#[cfg(feature = "debug")]
use super::debug::{RefCell, Ref, RefMut};
#[cfg(not(feature = "debug"))]
use std::cell::{RefCell, Ref, RefMut};
use std::any::Any;
use std::ops::{Deref, DerefMut};
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
    origin_node: WeakNode,
}

impl UserInputs {
    pub fn new(origin_node: WeakNode) -> Self {
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
            origin_node, 
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
        
        let origin_borrow = self.origin_node.upgrade().unwrap();

        self.hover_focus = origin_borrow.borrow().hit_detect(self.mouse);

        if self.left_mouse_pressed || !self.left_mouse_down {
            self.hoverhold_focus = self.hover_focus.clone();
            self.hoverhold_mouse = self.mouse;
            if self.left_mouse_pressed {
                self.lasttouch_focus = self.hover_focus.clone();
                self.lasttouch_mouse = self.mouse;
            }
        }
    }

    pub fn hoverhold_test(&self, node: &StrongNode) -> bool {
        node.borrow().contains_self(&self.hoverhold_focus)
    }

    pub fn hover_test(&self, node: &StrongNode) -> bool {
        node.borrow().contains_self(&self.hover_focus)
    }

    pub fn last_touch_test(&self, node: &StrongNode) -> bool {
        node.borrow().contains_self(&self.lasttouch_focus)
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

pub type StrongRef<T> = Rc<Borrow<T>>;
pub type WeakRef<T> = Weak<Borrow<T>>;
pub type WeakNode = WeakRef<NodeStore>;
pub type StrongNode = StrongRef<NodeStore>;

pub trait Node: AsAny {
    fn update(&mut self, user_inputs: &UserInputs);
    fn hit_detect(&self, pos: Vec2) -> Vec<WeakNode>;
}

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub trait New {
    type InType;
    fn new(data: Self::InType, node: WeakNode) -> Self;
}


#[derive(Default)]
struct NoNode;
impl Node for NoNode {
    fn update(&mut self, _user_inputs: &UserInputs) {panic!("NoNode used as node.")}
    fn hit_detect(&self, _pos: Vec2) -> Vec<WeakNode> {panic!("NoNode used as node.")}
}

fn temp_node() -> RefCell<Box<dyn Node + 'static>> {
    RefCell::new(Box::new(NoNode))
}


pub struct NodeStore {
    this: Option<WeakNode>, // This is not optional, None is only the case within function calls.
    node: RefCell<Box<dyn Node>>,
    parent: Option<WeakNode>, // This is only None if it is origin.
    pub children: Vec<StrongNode>
}

const SELF_THIS_INVALID: &str = "pointer to self dropped and upgrade impossible. This means `self.this` did not point to self.";
const INVALID_WEAK: &str = "pointer dropped and upgrade impossible.";
const DOWNCAST_FAILED: &str = "downcast failed. Either tree is malformed, or an incorrect type was supplied.";

impl NodeStore {
    pub fn origin<T: Node + New<InType = ()> + 'static>() -> StrongRef<Self> {
        Self::new::<T>((), None)
    }

    pub fn push_child<Child: Node + New + 'static>(&mut self, data: Child::InType) {
        self.children.push(
            Self::new::<Child>(data, Some(self.this.clone().expect(SELF_THIS_INVALID)))
        );
    }

    fn new<Child: Node + New + 'static>(data: Child::InType, parent: Option<WeakNode>) -> StrongRef<Self>{
        let result = Rc::new(Borrow::new(Self {
            this: None,
            node: temp_node(),
            parent,
            children: vec![]
        }));
        
        let weak = Rc::downgrade(&result);
        result.borrow_mut().this = Some(weak.clone());
        let value = Box::new(Child::new(data, weak));
        result.borrow_mut().node = RefCell::new(value);

        result
    }
    
    pub fn contains_self(&self, vec: &[WeakNode]) -> bool {
        let this = self.this.clone().unwrap().upgrade().unwrap_or_else(|| panic!("In `contains_self`, {SELF_THIS_INVALID}"));
        vec.iter().any(|d|
            Rc::ptr_eq(&this, &d.upgrade().unwrap_or_else(|| panic!("In `contains_self`, {INVALID_WEAK}")))
        )
    }

    pub fn weak_self(&self) -> WeakRef<Self> {
        self.this.clone().unwrap()
    }
    
    pub fn try_get_self<T: Node + 'static>(&self) -> Option<&T> {
        self.node.borrow().as_ref().as_any().downcast_ref()
    }
    
    pub fn try_get_self_mut<T: Node + Any>(&self) -> Option<RefMut<'_, T>> {
        // self.node.borrow_mut().as_mut().as_any_mut().downcast_mut()
        let guard = self.node.borrow_mut();
        if guard.as_any().is::<T>() {
            Some(RefMut::map(guard, |d| {
                d.as_any_mut().downcast_mut().unwrap()
            }))
        } else {
            None
        }
    }

    // fn get_button(&self) -> Option<RefMut<'_, Button>> {
    //     let guard = self.widget.borrow_mut();
    //     if guard.as_any_mut().is::<Button>() {
    //         Some(RefMut::map(guard, |w| {
    //             w.as_any_mut().downcast_mut::<Button>().unwrap()
    //         }))
    //     } else {
    //         None
    //     }
    // }
    
    pub fn get_self<T: Node + 'static>(&self) -> &T {
        self.try_get_self().unwrap_or_else(|| panic!("In `get_self`, {DOWNCAST_FAILED}"))
    }
    
    pub fn get_self_mut<T: Node + 'static>(&mut self) -> &mut T {
        self.try_get_self_mut().unwrap_or_else(|| panic!("In `get_self_mut`, {DOWNCAST_FAILED}"))
    }

    pub fn get_self_dyn(&self) -> &dyn Node {
        self.node.as_ref()
    }
    
    pub fn get_self_dyn_mut(&mut self) -> &mut Box<dyn Node> {
        &mut self.node
    }
    
    pub fn get_parent(&self) -> StrongNode {
        self.parent.clone().unwrap().upgrade().unwrap_or_else(|| panic!("In `get_parent`, {INVALID_WEAK}"))
    }
    
    pub fn update_children(&mut self, user_inputs: &UserInputs) {
        for child in &self.children {
            child.borrow_mut().get_self_dyn_mut().update(user_inputs);
        }
    }
}

struct Borrow<T: ?Sized> {
    cell: RefCell<T>
}

struct BRef<'a, T: ?Sized> {
    cell: Ref<'a, T>
}

struct BRefMut<'a, T: ?Sized> {
    cell: RefMut<'a, T>
}

impl<T> Borrow<T> {
    pub fn new(data: T) -> Self {
        Self { cell: RefCell::new(data) }
    }

    pub fn borrow(&self) -> BRef<'_, T> {
        BRef { cell: self.cell.borrow()}
    }

    fn borrow_mut(&self) -> BRefMut<'_, T> {
        BRefMut { cell: self.cell.borrow_mut() }
    }
}

impl<'a, T> Deref for BRef<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.cell.deref()
    }
}

impl<'a, T> Deref for BRefMut<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.cell.deref()
    }
}

impl<'a, T> DerefMut for BRefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.cell.deref_mut()
    }
}
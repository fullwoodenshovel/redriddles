use std::cell::RefCell;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};
mod user_inputs;
pub use user_inputs::{UserInputs, ShortcutInstruction, Shortcuts};

use macroquad::prelude::*;


struct AppContext {
    user_inputs: UserInputs,
    store: Store,
    shortcuts: Shortcuts
}

impl AppContext {
    fn get_handler(&'_ mut self) -> AppContextHandler<'_> {
        AppContextHandler { user_inputs: &self.user_inputs, store: &mut self.store, shortcuts: &mut self.shortcuts }
    }
}

pub struct AppContextHandler<'a> {
    pub user_inputs: &'a UserInputs,
    pub store: &'a mut Store,
    pub shortcuts: &'a mut Shortcuts
}

pub struct Store {
    store: HashMap<TypeId, Box<dyn Any>>
}

impl Store {
    fn new() -> Self {
        Self {
            store: HashMap::new()
        }
    }

    pub fn get<T: Any>(&self) -> &T {
        self.store
            .get(&TypeId::of::<T>())
            .unwrap_or_else(|| panic!("Data expected from AppContextGenHandler of a type that hasn't been entered.\nT = {}", std::any::type_name::<T>()))
            .downcast_ref()
            .unwrap()
    }

    pub fn get_mut<T: Any>(&mut self) -> &mut T {
        self.store
            .get_mut(&TypeId::of::<T>())
            .unwrap_or_else(|| panic!("Data expected from AppContextGenHandler of a type that hasn't been entered.\nT = {}", std::any::type_name::<T>()))
            .downcast_mut()
            .unwrap()
    }

    pub fn overwrite<T: Any>(&mut self, value: T) {
        *self.get_mut::<T>() = value;
    }

    #[allow(private_bounds)]
    pub fn set<T: Any + DerefMut>(&mut self, value: T::Target)
    where <T as Deref>::Target: Sized
    {
        **self.get_mut::<T>() = value;
    }

    #[allow(private_bounds)]
    pub fn value<T: Any + Deref>(&self) -> T::Target
    where T::Target: Copy
    {
        **self.get::<T>()
    }
}

pub struct AppContextGenHandler {
    store: Store
}

impl AppContextGenHandler {
    fn new() -> Self {
        Self {
            store: Store::new()
        }
    }

    pub fn push<T: Any>(&mut self, data: T) {
        self.push_boxed(Box::new(data));
    }
    
    pub fn push_boxed<T: Any>(&mut self, data: Box<T>) {
        if self.store.store.insert(TypeId::of::<T>(), data).is_some() {
            panic!("Data of the same type given to AppContextGenHandler.")
        }
    }

    fn into_context(self, user_inputs: UserInputs) -> AppContext {
        AppContext { user_inputs, store: self.store, shortcuts: Shortcuts::default() }
    }
}

impl Deref for AppContextGenHandler {
    type Target = Store;
    fn deref(&self) -> &Self::Target {
        &self.store
    }
}

impl DerefMut for AppContextGenHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.store
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

/// # Example usage:
/// ```no_run
/// use super::*;
///
/// pub struct Example {
///
/// }
///
/// impl NewNoOut for Example {
///     type InType = ();
///     fn new(_: Self::InType, _handler: &mut GenHandler) -> Self {
///         Self {
///
///         }
///     }
/// }
///
/// impl Node for Example {
///     fn update(&mut self, ctx: &mut AppContextHandler, node: &NodeStore) {
///
///     }
///
///     fn hit_detect(&mut self, pos: Vec2, node: &NodeStore, store: &mut Store) -> Vec<WeakNode> {
///         if self.rect.contains(pos) {
///             vec![node.get_weak()]
///         } else {
///             vec![]
///         }
///     }
/// }
/// ```
#[allow(private_bounds)]
pub trait Node: AsAny {
    fn update(&mut self, user_inputs: &mut AppContextHandler, node: &NodeStore);
    fn hit_detect(&mut self, pos: Vec2, node: &NodeStore, store: &mut Store) -> Vec<WeakNode>;
}

#[cfg(feature = "hit_detect_debug")]
trait AsAny: AsAnyDebug {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[cfg(not(feature = "hit_detect_debug"))]
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

#[cfg(feature = "hit_detect_debug")]
trait AsAnyDebug {
    fn get_name(&self) -> &'static str;
}

impl<A: Any + Node> AsAny for A {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl<A: AsAny + ?Sized> AsAnyExt for A {
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

#[cfg(feature = "hit_detect_debug")]
impl<A: AsAny + ?Sized> AsAnyDebug for A {
    fn get_name(&self) -> &'static str {
        std::any::type_name::<A>()
    }
}

pub trait NewInOut {
    type InType;
    type OutType;
    fn new(data: Self::InType, handler: &mut GenHandler) -> (Self::OutType, Self);
}

pub trait NewNoOut {
    type InType;
    fn new(data: Self::InType, handler: &mut GenHandler) -> Self;
}

impl<T: NewNoOut> NewInOut for T {
    type InType = T::InType;
    type OutType = ();
    fn new(data: Self::InType, handler: &mut GenHandler) -> (Self::OutType, Self) {
        ((), T::new(data, handler))
    }
}

pub trait New {
    fn new(handler: &mut GenHandler) -> Self;
}

impl<T: New> NewNoOut for T {
    type InType = ();
    fn new(_: Self::InType, handler: &mut GenHandler) -> Self {
        T::new(handler)
    }
}

pub struct GenHandler<'a> {
    children: Vec<Rc<NodeStore>>,
    ctx: &'a mut AppContextGenHandler
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
    pub fn hit_detect(&self, pos: Vec2, store: &mut Store) -> Vec<WeakNode> {
        self.this.node.borrow_mut().hit_detect(pos, self.this, store)
    }

    pub fn update(&self, ctx: &mut AppContextHandler) {
        self.this.node.borrow_mut().update(ctx, self.this);
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

impl GenHandler<'_> {
    pub fn push_child<Child: Node + New + 'static>(&mut self) {
        self.push_child_io::<Child>(())
    }

    pub fn push_child_io<Child: Node + NewInOut + 'static>(&mut self, data: Child::InType) -> Child::OutType {
        let (out, node) = NodeStore::new::<Child>(data, self.ctx);
        self.children.push(
            node
        );
        out
    }

    pub fn push_data<T: Any>(&mut self, data: T) {
        self.ctx.push(data);
    }

    pub fn push_data_boxed<T: Any>(&mut self, data: Box<T>) {
        self.ctx.push_boxed(data);
    }
}

impl NodeStore {
    pub fn origin<T: Node + NewInOut<InType = (), OutType = ()> + 'static>(ctx: &mut AppContextGenHandler) -> Origin {
        Origin {node: Self::new::<T>((), ctx).1 }
    }

    fn new<Child: Node + NewInOut + 'static>(data: Child::InType, ctx: &mut AppContextGenHandler) -> (Child::OutType, StrongNode) {
        let mut handler = GenHandler { children: vec![], ctx };
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

    pub fn get_children(&'_ self) -> Vec<Handler<'_>>{
        self.children.iter().map(|c| Handler::new(c.as_ref())).collect()
    }

    pub fn update_children(&self, ctx: &mut AppContextHandler) {
        for child in self.get_children() {
            child.update(ctx)
        }
    }

    pub fn hit_detect_children_and_self(&self, pos: Vec2, store: &mut Store) -> Vec<WeakNode>{
        let mut result = vec![];
        for child in self.get_children() {
            result = child.hit_detect(pos, store);
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

pub struct Frame {
    origin: Origin,
    ctx: AppContext
}

impl Frame {
    pub fn new<T: Node + NewInOut<InType = (), OutType = ()> + 'static>() -> Self {
        let mut ctx = AppContextGenHandler::new();
        let origin = NodeStore::origin::<T>(&mut ctx);
        let user_inputs = UserInputs::new(&origin);
        let ctx = ctx.into_context(user_inputs);
        Self {
            origin,
            ctx
        }
    }

    pub async fn update(&mut self) {
        self.ctx.user_inputs.update(&mut self.ctx.store, &self.ctx.shortcuts);
        #[cfg(feature = "hit_detect_debug")]
        if !(self.ctx.user_inputs.prev_hover_focus.len() == self.ctx.user_inputs.hover_focus.len() &&
            self.ctx.user_inputs.prev_hover_focus.iter().enumerate().all(|(i, d)| d.ptr_eq(&self.ctx.user_inputs.hover_focus[i])))
        {
            println!("{}", self.ctx.user_inputs.hover_focus.iter().map(|d| d
                .upgrade()
                .unwrap()
                .node
                .borrow()
                .get_name()
                .rsplit_once(":")
                .unwrap()
                .1
            ).collect::<Vec<_>>().join("  <--  "));
        }
        self.origin.get_handler().update(&mut self.ctx.get_handler());
        next_frame().await;
    }
}
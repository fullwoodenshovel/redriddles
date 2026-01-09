use std::cell::RefCell as StdCell;
use std::cell::{Ref as StdRef, RefMut as StdMut};
use std::ops::{Deref, DerefMut};

pub struct RefCell<T: ?Sized> {
    cell: StdCell<T>
}

impl<T> RefCell<T> {
    pub fn new(data: T) -> Self {
        Self {
            cell: StdCell::new(data)
        }
    }

    pub fn borrow(&self) -> Ref<'_, T> {
        println!("Borrow");
        Ref { cell: self.cell.borrow() }
    }

    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        println!("Borrow mut");
        RefMut { cell: self.cell.borrow_mut() }
    }
}

pub struct Ref<'a, T> {
    cell: StdRef<'a, T>
}

impl<'a, T> Drop for Ref<'a, T> {
    fn drop(&mut self) {
        println!("Dropped borrow")
    }
}

impl<'a, T> Deref for Ref<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.cell.deref()
    }
}

pub struct RefMut<'a, T> {
    cell: StdMut<'a, T>
}

impl<'a, T> Drop for RefMut<'a, T> {
    fn drop(&mut self) {
        println!("Dropped mut borrow")
    }
}

impl<'a, T> Deref for RefMut<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.cell.deref()
    }
}

impl<'a, T> DerefMut for RefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.cell.deref_mut()
    }
}
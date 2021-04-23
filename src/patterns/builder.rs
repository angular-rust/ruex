#![allow(dead_code)]
#![allow(unused_variables)]
use std::{cell::RefCell, ops::Deref};
use std::rc::Rc;

pub struct Builder<T> {
    inner: RefCell<Option<Rc<T>>>,
}

impl<T> Default for Builder<T>
where
    T: Default + Clone,
{
    fn default() -> Self {
        Self {
            inner: RefCell::new(Some(Rc::new(Default::default()))),
        }
    }
}

impl<T> Builder<T>
where
    T: Default + Clone,
{
    pub fn build(&self) -> Option<T> {
        let mut inner = self.inner.borrow_mut();
        match inner.as_ref() {
            Some(val) => {
                let ret = val.clone();
                *inner = None;
                Some((*ret).clone())
            }
            None => None,
        }
    }
}

pub trait Construction<T> {
    fn construct() -> Builder<T>;
}

impl<T> Construction<T> for T
where
    T: Default + Clone,
{
    fn construct() -> Builder<T> {
        Builder::<T>::default()
    }
}

// #[derive(Default, Copy, Clone)]
// pub struct Button;

// impl Builder<Button> {
//     pub fn title(&self, _: &str) -> &Self {
//         self
//     }

//     pub fn background(&self, _: &str) -> &Self {
//         self
//     }

//     pub fn enabled(&self, _: bool) -> &Self {
//         self
//     }
// }

// fn main() {
//     let a = Builder::<Button>::default()
//         .title("My button")
//         .background("#FF0044")
//         .enabled(true)
//         .build().unwrap_or_default(); // OUCH
//         // .build().expect("The chest is empty"); // Specify message
//         // .build().unwrap(); // Just panic
// }

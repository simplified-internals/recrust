use std::any::Any;

use dioxus_native::prelude::Element;

pub fn create_element<T, P>(tag: T, props: Vec<(&str, Box<dyn Any>)>) -> Element
where
    T: Fn(P) -> Element,
{
    todo!()
}

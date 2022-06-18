use std::fmt::Display;

pub trait Entity: Display {}

pub trait Item: Entity {
    fn use_item(&mut self);
}

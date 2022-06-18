use std::fmt::Display;

use crate::traits::{Entity, Item};

pub trait Location: Entity {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn available_locations(&self) -> Vec<Box<dyn Location>> {
        Vec::new()
    }
    fn available_items(&self) -> Vec<Box<dyn Item>> {
        Vec::new()
    }
}

struct BasicLocation {
    name: String,
    description: String,
}

impl Display for BasicLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Entity for BasicLocation {}

impl Location for BasicLocation {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn description(&self) -> String {
        self.description.clone()
    }
}

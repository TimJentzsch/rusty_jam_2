use std::fmt::Display;

trait Entity: Display {}

trait Item: Entity {
    fn use_item(&mut self);
}

trait Location: Entity {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn available_locations(&self) -> Vec<Box<dyn Location>>;
    fn available_items(&self) -> Vec<Box<dyn Item>>;
}

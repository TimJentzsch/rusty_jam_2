use locations::{BasicLocation, Location};

mod locations;
mod traits;

fn main() {
    let home = BasicLocation::new("Home".to_string(), "A lovely and cozy place.".to_string());

    let cur_location: Box<dyn Location> = Box::new(home);

    println!("{cur_location}\n{}", cur_location.description());
}

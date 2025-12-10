use std::fmt::Debug;

pub mod snoopy;

pub trait Waypoint: Debug + Clone {
    fn new(x: i32, y: i32, z: i32) -> Self;
}

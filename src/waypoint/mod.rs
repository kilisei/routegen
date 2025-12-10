use std::fmt::Debug;

pub mod snoopy;

pub trait Waypoint: Debug + Clone {
   fn new(x: u16,y: u16, z: u16) -> Self;
   fn position(&self) -> (u16, u16, u16);
}


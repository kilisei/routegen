use crate::waypoint::Waypoint;
use clap::ValueEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum RouteFormat {
    Snoopy,
}

#[derive(Debug, Clone)]
pub struct Route<W: Waypoint> {
    pub format: RouteFormat,
    pub waypoints: Vec<W>,
}

impl<W: Waypoint> Route<W> {
    pub fn new(format: RouteFormat, waypoints: Vec<W>) -> Self {
        Self { format, waypoints }
    }

    pub fn add_waypoint(&mut self, wp: W) {
        self.waypoints.push(wp);
    }
}

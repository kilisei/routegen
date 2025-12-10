use crate::waypoint::Waypoint;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RouteType {
    Snoopy,
}

#[derive(Debug, Clone)]
pub struct Route<W: Waypoint> {
    pub format: RouteType,
    pub waypoints: Vec<W>,
}

impl<W: Waypoint> Route<W> {
    pub fn new(format: RouteType, waypoints: Vec<W>) -> Self {
        Self { format, waypoints }
    }

    pub fn add_waypoint(&mut self, wp: W) {
        self.waypoints.push(wp);
    }
}

use crate::{
    cli::{args::Args, coordinate::Coordinate},
    route::{self, route::Route},
    waypoint::{self, Waypoint, snoopy::SnoopyWaypoint},
};

pub fn generate_route<T: Waypoint>(args: &Args) -> Route<T> {
    let mut route = Route::new(args.output_format, Vec::with_capacity(args.length));

    let mut ore_waypoints: Vec<Coordinate> = Vec::new();

    ore_waypoints.push(Coordinate::new(1, 1, 1));
    ore_waypoints.push(Coordinate::new(2, 2, 2));
    ore_waypoints.push(Coordinate::new(3, 3, 3));
    ore_waypoints.push(Coordinate::new(4, 4, 4));

    while route.waypoints.len() < args.length {
        let wp = T::new(1, 1, 1);
        route.add_waypoint(wp);
    }

    route
}

use std::collections::HashSet;

use crate::{cli::{args::Args, coordinate::Coordinate}};

pub fn generate_route(args: &Args) -> Vec<Coordinate> {
    let mut ores: Vec<Coordinate> = Vec::new();
    ores.push(Coordinate::new(100, 100, 100));
    ores.push(Coordinate::new(200, 200, 200));
    // ores.push(Coordinate::new(300, 300, 300));
    ores.push(Coordinate::new(400, 400, 400));
    // ores.push(Coordinate::new(500, 500, 500));

    if ores.is_empty() || args.length == 0 {
        return Vec::new();
    }

    let mut route: Vec<Coordinate> = Vec::with_capacity(args.length);
    let mut visited: HashSet<Coordinate> = HashSet::new();
    let mut current = args.origin.closest(&ores).expect("Failed to get closest to");
    route.push(current);
    visited.insert(current);

    while route.len() < args.length {
        println!("<<<<<<<<<<<<<<<<<>>>>>>>>>>>>>>>>>>>");
        dbg!(&current);
        dbg!(&visited);

        match next_waypoint(&ores, &current, &visited) {
            Some(next) => {
                current = next.clone();
                visited.insert(next.clone());
                route.push(next);
            }
            None => break,
        }
    }

    dbg!(&route);

    route
}

fn next_waypoint(
    ores: &Vec<Coordinate>,
    current: &Coordinate,
    visited: &HashSet<Coordinate>,
) -> Option<Coordinate> {
    ores.iter()
        .filter(|coordinate| !visited.contains(*coordinate))
        .min_by(|a, b| {
            let da = current.distance_to(a);
            let db = current.distance_to(b);
            da.partial_cmp(&db).unwrap()
        })
        .map(|c| c.clone())
}

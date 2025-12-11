use std::{collections::HashSet, fs};

use serde::{Deserialize, Serialize};

use crate::{
    cli::{args::Args, coordinate::Coordinate},
    ore::ore::Ore,
};

#[derive(Serialize, Deserialize, Clone, Copy)]
struct Center {
    x: i32,
    y: i32,
    z: i32,
}
#[derive(Serialize, Deserialize, Clone, Copy)]
struct Cluster {
    center: Center,
    count: usize,
}
fn get_file_name(ore: &Ore) -> String {
    match ore {
        Ore::Coal => "coal".to_string(),
    }
}

fn get_ores_coordinates(ore: &Ore) -> Vec<Coordinate> {
    let path = format!("./assets/{}_clusters.json", get_file_name(ore));
    let ores = fs::read_to_string(path).unwrap();
    let ores: Vec<Cluster> = serde_json::from_str(&ores).unwrap();

    ores.into_iter()
        .map(|o| {
            let cord = Coordinate::new(o.center.x, o.center.y, o.center.z);
            cord
        })
        .collect()
}

pub fn generate_route(args: &Args) -> Vec<Coordinate> {
    let ores = get_ores_coordinates(&args.ore);

    if ores.is_empty() || args.length == 0 {
        return Vec::new();
    }

    let mut route: Vec<Coordinate> = Vec::with_capacity(args.length);
    let mut visited: HashSet<Coordinate> = HashSet::with_capacity(args.length);
    let mut current = args
        .origin
        .closest(&ores)
        .expect("Failed to get closest to origin");
    route.push(current);
    visited.insert(current);

    while route.len() < args.length {
        match next_waypoint(&ores, &current, &visited, true) {
            Some(next) => {
                current = next.clone();
                visited.insert(next.clone());
                route.push(next);
            }
            None => break,
        }
    }

    route
}

fn next_waypoint(
    ores: &[Coordinate],
    current: &Coordinate,
    visited: &HashSet<Coordinate>,
    only_mf: bool,
) -> Option<Coordinate> {
    ores.iter()
        .filter(|coordinate| {
            if only_mf {
                coordinate.y < 64 && !visited.contains(coordinate)
            } else {
                coordinate.y > 64 || !visited.contains(coordinate)
            }
        })
        .min_by(|a, b| {
            let da = current.distance_to(a);
            let db = current.distance_to(b);
            da.partial_cmp(&db).unwrap()
        })
        .cloned()
}

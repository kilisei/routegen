use serde::{Deserialize, Serialize};

use crate::waypoint::Waypoint;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnoopyWaypointOptions {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnoopyWaypoint {
    pub x: i32,
    pub y: i32,
    pub z: i32,

    pub r: u8,
    pub g: u8,
    pub b: u8,

    pub options: SnoopyWaypointOptions,
}

impl Waypoint for SnoopyWaypoint {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            x: x,
            y: y,
            z: z,

            r: 0,
            g: 0,
            b: 0,

            options: SnoopyWaypointOptions {
                name: String::new(),
            },
        }
    }
}

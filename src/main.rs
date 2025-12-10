use clap::Parser;

use crate::{cli::args::Args, route::route::{Route, RouteType}, waypoint::{Waypoint, snoopy::SnoopyWaypoint}};

mod waypoint;
mod route;
mod cli;
mod ore;

fn main() {
    let args = Args::parse();

    dbg!(&args);
}


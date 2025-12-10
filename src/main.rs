use std::{fs::File, io::Result};

use clap::Parser;

use crate::{
    cli::args::Args,
    route::{generate::generate_route, route::Route}, waypoint::{Waypoint, snoopy::SnoopyWaypoint},
};

mod cli;
mod ore;
mod route;
mod waypoint;

fn main() -> Result<()> {
    let args = Args::parse();

    let route = generate_route(&args);

    let waypoints = route.iter().map(|i| {
        SnoopyWaypoint::new(i.x, i.y, i.z)
    }).collect::<Vec<SnoopyWaypoint>>();

    let route = Route::new(args.output_format, waypoints);

    match &args.output_file {
        Some(path) => {
            let buff = File::create(path)?;
            serde_json::to_writer(buff, &route.waypoints)?;
        }
        None => {
            let buff = std::io::stdout();
            serde_json::to_writer_pretty(buff, &route.waypoints)?
        }
    }

    Ok(())
}

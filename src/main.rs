use std::{fs::File, io::Result};

use clap::Parser;
use serde_json;

use crate::{
    cli::args::Args,
    route::{generate::generate_route, route::Route},
    waypoint::snoopy::SnoopyWaypoint,
};

mod cli;
mod ore;
mod route;
mod waypoint;

fn main() -> Result<()> {
    let args = Args::parse();

    let route = generate_route(&args);

    dbg!(route);
    //
    //
    // match &args.output_file {
    //     Some(path) => {
    //         let buff = File::create(path)?;
    //         serde_json::to_writer(buff, &route.waypoints)?;
    //     }
    //     None => {
    //         let buff = std::io::stdout();
    //         serde_json::to_writer(buff, &route.waypoints)?
    //     }
    // }

    Ok(())
}

use clap::Parser;

use crate::cli::args::Args;

mod cli;
mod ore;
mod route;
mod waypoint;

fn main() {
    let args = Args::parse();

    dbg!(&args);
}

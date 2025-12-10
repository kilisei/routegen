use std::path::PathBuf;

use clap::Parser;
use crate::{cli::coordinate::Coordinate, ore::ore::Ore};

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    #[arg(long, default_value_t = Coordinate::new(513,69,513))]
    pub origin: Coordinate,

    #[arg(short, long, default_value_t = 128usize)]
    pub length: usize,

    #[arg(short, long, default_value_t = 7)]
    pub min_vein_distance: usize,

    #[arg(long, rename_all = "lower", value_enum)]
    pub ore: Ore,

    #[arg(short, long)]
    pub output_file: Option<PathBuf>
}


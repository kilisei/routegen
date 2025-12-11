use std::path::PathBuf;

use crate::route::route::RouteFormat;
use crate::{cli::coordinate::Coordinate, ore::ore::Ore};
use clap::Parser;
use clap::ValueEnum;

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum Area {
    MagamFields,
    PrecursorRemnants,
    MithrilDepostits,
    Jungle,
    GoblinHoldout,
}

impl Area {
    pub fn all() -> Vec<Self> {
        vec![
            Self::MagamFields,
            Self::MithrilDepostits,
            Self::PrecursorRemnants,
            Self::GoblinHoldout,
            Self::Jungle,
        ]
    }

    pub fn get_bounds(&self) -> Vec<Coordinate> {
        match self {
            Area::MagamFields => vec![],
            Area::MithrilDepostits => vec![],
            Area::PrecursorRemnants => vec![],
            Area::GoblinHoldout => vec![],
            Area::Jungle => vec![],
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    #[arg(long, default_value_t = Coordinate::new(513,69,513))]
    pub origin: Coordinate,

    #[arg(long, rename_all = "lower", value_enum, default_value_t = RouteFormat::Snoopy)]
    pub output_format: RouteFormat,

    #[arg(short, long, default_value_t = 128usize)]
    pub length: usize,

    #[arg(long, default_value_t = 44)]
    pub min_ores_per_vein: usize,

    #[arg(long, default_value_t = 7)]
    pub min_vein_distance: usize,

    #[arg(short, long, value_enum, num_args = 1.., rename_all = "lower", default_values_t = Area::all())]
    pub area: Vec<Area>,

    #[arg(long, rename_all = "lower", value_enum)]
    pub ore: Ore,

    #[arg(short, long)]
    pub output_file: Option<PathBuf>,
}

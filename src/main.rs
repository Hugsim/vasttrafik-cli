mod api;
mod print_helpers;

use std::error::Error;

use api::endpoints::{location_name::*, trip::call_trip};

use clap::{Parser, Subcommand};
use log::info;

#[derive(Parser, Debug)]
struct Args {
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
    #[clap(subcommand)]
    subcommand: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    Search {
        #[clap(short, long)]
        station: String,
        #[clap(short, long, default_value_t = 10)]
        count: usize,
    },
    Plan {
        #[clap(short, long)]
        from: String,
        #[clap(short, long)]
        to: String,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    let mut token = api::get_token()?;

    match args.subcommand {
        SubCommand::Search { station, count } => {
            info!(
                "Searching for the station {}, returning {} answers.",
                station, count
            );
            let places = call_location_name(&mut token, &station)?
                .location_list
                .stop_location;

            for p in &places[..count] {
                println!("{}", p.name);
            }
        }
        SubCommand::Plan { from, to } => {
            info!("Planning trip from {} to {}.", from, to);
            let from_id = &&call_location_name(&mut token, &from)?
                .location_list
                .stop_location[0]
                .id;
            let to_id = &&call_location_name(&mut token, &to)?
                .location_list
                .stop_location[0]
                .id;
            let res = call_trip(&mut token, from_id, to_id)?;
            let first_trip = &res.trip_list.trips[0];
            for leg in &first_trip.legs {
                println!("{}", leg);
            }
        }
    }

    Ok(())
}

mod api;

use std::error::Error;

use api::endpoints::{location_name::*, trip::call_trip};

use clap::{Parser, Subcommand};

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
        #[clap(short, long)]
        count: Option<usize>,
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

    let mut token = api::get_token()?;

    match args.subcommand {
        SubCommand::Search { station, count } => {
            let places = call_location_name(&mut token, &station)?
                .location_list
                .stop_location;
            let count = match count {
                Some(count) => count,
                None => places.len(),
            };

            for p in &places[..count] {
                println!("{}", p.name);
            }
        }
        SubCommand::Plan { from, to } => {
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
                let actual_dep_time = match &leg.origin.rt_time {
                    Some(actual_time) => format!(" ({})", actual_time),
                    None => "".to_string(),
                };
                let actual_arr_time = match &leg.destination.rt_time {
                    Some(actual_time) => format!(" ({})", actual_time),
                    None => "".to_string(),
                };
                println!(
                    "{} @ {}{} --[{}]--> {} @ {}{}",
                    leg.origin.name.split(",").collect::<Vec<&str>>()[0],
                    leg.origin.time,
                    actual_dep_time,
                    leg.name,
                    leg.destination.name.split(",").collect::<Vec<&str>>()[0],
                    leg.destination.time,
                    actual_arr_time,
                );
            }
        }
    }

    Ok(())
}

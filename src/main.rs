mod api;

use std::error::Error;

use api::endpoints::{location_name::*, trip::call_trip};

use clap::{Parser, Subcommand};
use colored::Colorize;

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
                // TODO: Should probably move this out into impl Leg
                let fg_colour = match &leg.fg_color {
                    Some(col) => {
                        let col = &col[1..];
                        let col = hex::decode(col)?;
                        colored::Color::TrueColor {
                            r: col[0],
                            g: col[1],
                            b: col[2],
                        }
                    }
                    None => colored::Color::White,
                };
                let bg_colour = match &leg.bg_color {
                    Some(col) => {
                        let col = &col[1..];
                        let col = hex::decode(col)?;
                        colored::Color::TrueColor {
                            r: col[0],
                            g: col[1],
                            b: col[2],
                        }
                    }
                    None => colored::Color::Black,
                };
                let dep_time_str = match &leg.origin.rt_time {
                    Some(actual_time) => {
                        format!("{} {}", leg.origin.time.strikethrough(), actual_time.bold())
                    }
                    None => leg.origin.time.to_string(),
                };
                let arr_time_str = match &leg.destination.rt_time {
                    Some(actual_time) => {
                        format!(
                            "{} {}",
                            leg.destination.time.strikethrough(),
                            actual_time.bold()
                        )
                    }
                    None => leg.destination.time.to_string(),
                };
                let dep_track_str = match &leg.origin.track {
                    Some(track) => match &leg.origin.rt_track {
                        Some(actual_track) => format!("{} {}", track.strikethrough(), actual_track),
                        None => format!("{}", track),
                    },
                    None => "".to_string(),
                };
                let arr_track_str = match &leg.destination.track {
                    Some(track) => match &leg.destination.rt_track {
                        Some(actual_track) => format!("{} {}", track.strikethrough(), actual_track),
                        None => format!("{}", track),
                    },
                    None => "".to_string(),
                };
                println!(
                    "{} {} @ {} --{}--> {} {} @ {}",
                    leg.origin.name.split(",").collect::<Vec<&str>>()[0],
                    dep_track_str,
                    dep_time_str,
                    format!("[{}]", leg.name)
                        .color(fg_colour)
                        .on_color(bg_colour),
                    leg.destination.name.split(",").collect::<Vec<&str>>()[0],
                    arr_track_str,
                    arr_time_str,
                );
            }
        }
    }

    Ok(())
}

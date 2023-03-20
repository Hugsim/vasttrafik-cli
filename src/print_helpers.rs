use std::fmt;

use colored::{Color, Colorize};

use crate::api::endpoints::trip::Leg;

pub fn string_to_color(color: &str) -> colored::Color {
    let color = &color[1..];
    let color = hex::decode(color).expect("Error in parsing color!");
    colored::Color::TrueColor {
        r: color[0],
        g: color[1],
        b: color[2],
    }
}

impl fmt::Display for Leg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fg_colour = self.fg_color_or(Color::White);
        let bg_colour = self.bg_color_or(Color::White);
        let dep_time_str = match &self.origin.rt_time {
            Some(actual_time) => {
                format!(
                    "{} {}",
                    self.origin.time.strikethrough(),
                    actual_time.bold()
                )
            }
            None => self.origin.time.to_string(),
        };
        let arr_time_str = match &self.destination.rt_time {
            Some(actual_time) => {
                format!(
                    "{} {}",
                    self.destination.time.strikethrough(),
                    actual_time.bold()
                )
            }
            None => self.destination.time.to_string(),
        };
        let dep_track_str = match &self.origin.track {
            Some(track) => match &self.origin.rt_track {
                Some(actual_track) => format!("{} {}", track.strikethrough(), actual_track),
                None => format!("{}", track),
            },
            None => "".to_string(),
        };
        let arr_track_str = match &self.destination.track {
            Some(track) => match &self.destination.rt_track {
                Some(actual_track) => format!("{} {}", track.strikethrough(), actual_track),
                None => format!("{}", track),
            },
            None => "".to_string(),
        };
        write!(
            f,
            "{} {} @ {} --{}--> {} {} @ {}",
            self.origin.name.split(",").collect::<Vec<&str>>()[0],
            dep_track_str,
            dep_time_str,
            format!("[{}]", self.name)
                .color(fg_colour)
                .on_color(bg_colour),
            self.destination.name.split(",").collect::<Vec<&str>>()[0],
            arr_track_str,
            arr_time_str,
        )
    }
}

impl Leg {
    pub fn fg_color_or(&self, default: colored::Color) -> colored::Color {
        match &self.fg_color {
            Some(fg_color) => string_to_color(fg_color),
            None => default,
        }
    }
    pub fn bg_color_or(&self, default: colored::Color) -> colored::Color {
        match &self.bg_color {
            Some(bg_color) => string_to_color(bg_color),
            None => default,
        }
    }
}

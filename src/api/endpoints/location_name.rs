use reqwest::Error;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_number_from_string;
use serde_this_or_that::as_i64;

use crate::api::types::Token;

use super::call_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub id: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub idx: i32,
    pub name: String,
    pub lon: String,
    pub lat: String,
    pub weight: Option<i32>,
    pub track: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coord {
    #[serde(deserialize_with = "as_i64")]
    pub idx: i64,
    pub name: String,
    pub lon: String,
    pub lat: String,
    #[serde(rename = "type")]
    pub coord_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationList {
    #[serde(rename = "noNamespaceSchemaLocation")]
    no_namespace_schema_location: String,
    pub servertime: String,
    pub serverdate: String,
    #[serde(rename = "StopLocation")]
    pub stop_location: Vec<Location>,
    #[serde(rename = "CoordLocation")]
    pub coord_location: Option<Vec<Coord>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationName {
    #[serde(rename = "LocationList")]
    pub location_list: LocationList,
}

pub fn call_location_name(token: &mut Token, location: &str) -> Result<LocationName, Error> {
    let url = "https://api.vasttrafik.se/bin/rest.exe/v2/location.name";
    call_json(token, url, [("format", "json"), ("input", location)])
}

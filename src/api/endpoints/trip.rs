use reqwest::Error;
use serde::{Deserialize, Deserializer, Serialize};
use serde_aux::prelude::bool_true;
use serde_json::Value;
use serde_this_or_that::as_bool;

use crate::api::types::Token;

use super::call_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    pub name: String,
    #[serde(rename = "routeIdx")]
    pub route_index: Option<String>,
    #[serde(rename = "$", default)]
    pub dollar_sign: String,
    #[serde(deserialize_with = "as_bool", default)]
    pub cancelled: bool,
    pub track: Option<String>,
    #[serde(rename = "rtTrack")]
    pub rt_track: Option<String>,
    #[serde(rename = "type")]
    pub stop_type: String,
    #[serde(rename = "Notes")]
    pub notes: Option<Notes>,
    pub id: String,
    #[serde(rename = "rtDate")]
    pub rt_date: Option<String>,
    #[serde(rename = "rtTime")]
    pub rt_time: Option<String>,
    pub directdate: Option<String>,
    pub directtime: Option<String>,
    pub date: String,
    pub time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub priority: String,
    pub severity: String,
    pub key: String,
    #[serde(rename = "$")]
    pub msg: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notes {
    #[serde(rename = "Note", deserialize_with = "de_obj_or_array")]
    pub note: Vec<Note>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ref {
    #[serde(rename = "ref")]
    pub reference: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Leg {
    pub id: Option<String>,
    pub name: String,
    pub sname: Option<String>,

    #[serde(rename = "type")]
    pub leg_type: String,
    #[serde(rename = "Origin")]
    pub origin: Position,
    #[serde(rename = "Destination")]
    pub destination: Position,
    pub direction: Option<String>,

    #[serde(rename = "fgColor")]
    pub fg_color: Option<String>,
    #[serde(rename = "bgColor")]
    pub bg_color: Option<String>,
    pub stroke: Option<String>,

    #[serde(rename = "Notes")]
    pub notes: Option<Notes>,

    #[serde(deserialize_with = "as_bool", default)]
    pub booking: bool,
    #[serde(deserialize_with = "as_bool", default)]
    pub cancelled: bool,
    #[serde(deserialize_with = "as_bool", default)]
    pub reachable: bool,
    #[serde(deserialize_with = "as_bool", default)]
    pub night: bool,
    pub accessibility: Option<String>,

    #[serde(rename = "JourneyDetailRef")]
    pub journey_detail_ref: Option<Ref>,
    #[serde(rename = "GeometryRef")]
    pub geometry_ref: Option<Ref>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trip {
    #[serde(rename = "Leg", deserialize_with = "de_obj_or_array")]
    pub legs: Vec<Leg>,
    #[serde(
        rename = "travelWarranty",
        deserialize_with = "as_bool",
        default = "bool_true"
    )]
    pub travel_warranty: bool,
    #[serde(deserialize_with = "as_bool", default = "bool_true")]
    pub valid: bool,
    #[serde(deserialize_with = "as_bool", default = "bool_true")]
    pub alternative: bool,
    #[serde(rename = "type")]
    pub trip_type: Option<String>,
}

fn de_obj_or_array<'de, T, D>(deserializer: D) -> Result<Vec<T>, <D as Deserializer<'de>>::Error>
where
    D: Deserializer<'de>,
    for<'df> T: Deserialize<'df>,
{
    let s: Value = serde::de::Deserialize::deserialize(deserializer)?;
    match s {
        arr @ Value::Array(_) => Ok(serde_json::from_value::<Vec<T>>(arr).unwrap()),
        map @ Value::Object(_) => Ok(vec![serde_json::from_value::<T>(map).unwrap()]),
        _ => Err(serde::de::Error::custom("got neither obj nor array")),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ManyTrips {
    #[serde(rename = "errorText")]
    pub error_text: Option<String>,
    pub error: Option<String>,
    pub serverdate: String,
    pub servertime: String,
    #[serde(rename = "Trip", deserialize_with = "de_obj_or_array")]
    pub trips: Vec<Trip>,
    #[serde(rename = "noNamespaceSchemaLocation")]
    pub no_namespace_schema_location: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TripList {
    #[serde(rename = "TripList")]
    pub trip_list: ManyTrips,
}

pub fn call_trip(token: &mut Token, from: &str, to: &str) -> Result<TripList, Error> {
    let url = "https://api.vasttrafik.se/bin/rest.exe/v2/trip";

    call_json::<_, TripList>(
        token,
        url,
        [("format", "json"), ("originId", from), ("destId", to)],
    )
}

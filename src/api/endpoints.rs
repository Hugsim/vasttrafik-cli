use std::time::Duration;

use reqwest::{blocking, Error};
use serde::{Deserialize, Serialize};

use crate::api::get_token;

use super::types::Token;

pub mod location_name;
pub mod trip;

fn call<T>(token: &mut Token, url: &str, query: T) -> Result<reqwest::blocking::Response, Error>
where
    T: Serialize,
{
    let res = blocking::Client::new()
        .get(url)
        .bearer_auth(&token.token)
        .query(&query)
        .send();
    match res {
        Ok(res) => Ok(res),
        Err(err) => {
            eprintln!(
                "Error in calling API! Trying again in a second...\n{}",
                err.to_string()
            );
            std::thread::sleep(Duration::from_secs(1));
            *token = get_token()?;
            call(token, url, query)
        }
    }
}

fn call_string<T>(token: &mut Token, url: &str, query: T) -> Result<String, Error>
where
    T: Serialize,
{
    call(token, url, query)?.text()
}

fn call_json<T, R>(token: &mut Token, url: &str, query: T) -> Result<R, Error>
where
    T: Serialize,
    for<'de> R: Deserialize<'de>,
    R: std::fmt::Debug,
{
    call(token, url, query)?.json::<R>()
}

use reqwest::{self, blocking, Error};

use base64::{engine::general_purpose, Engine};

pub mod endpoints;
mod types;

use types::*;

fn get_client_secret() -> String {
    // TODO: Should save these to file or similar, so they're not included in the program.
    let client_key = "273_8_fIj_w1i7t9FjeUi7p7vmga";
    let client_secret = "mYB1CVxKfRx90ntnBX1nH7C4Dk0a";
    let client_secret = format!("{}:{}", client_key, client_secret);
    let client_secret = general_purpose::STANDARD.encode(client_secret);
    client_secret
}
fn get_client_scope() -> String {
    let client_scope = "device_should";
    client_scope.to_owned()
}

pub fn get_token() -> Result<Token, Error> {
    let token_url = "https://api.vasttrafik.se/token";

    let params = [
        ("grant_type", String::from("client_credentials")),
        ("scope", get_client_scope()),
    ];

    let res = blocking::Client::new()
        .post(token_url)
        .form(&params)
        .header(
            "Authorization",
            String::from("Basic ") + &get_client_secret(),
        )
        .send()?
        .json::<TokenResponse>()?;

    let token = res.access_token;

    return Ok(Token::new(token));
}

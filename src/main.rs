mod headers;
mod config;
mod authorized_data;

#[macro_use]
extern crate rocket;

use std::str::{from_utf8};
use serenity::model::prelude::{Interaction};
use crate::authorized_data::AuthorizedRequest;
use crate::config::Config;

#[post("/api/interactions", format = "application/json", data = "<message>")]
fn api_interactions<'a>(message: AuthorizedRequest) -> Result<&'a str, ()> {
    let message_string = match from_utf8(message.data.as_slice()) {
        Ok(val) => { val }
        Err(err) => { return Err(()); }
    };
    let interaction: Interaction = serde_json::from_str(message_string).unwrap();
    return match interaction {
        Interaction::Ping(_) => { Ok("{\"type\": 1}") }
        _ => {
            Ok("{
            \"type\": 4,
            \"data\": {
                \"tts\": False,
                \"content\": \"Congrats on sending your command!\",
                \"embeds\": [],
                \"allowed_mentions\": { \"parse\": [] }
            }
        }")
        }
    };
}

#[get("/")]
fn index() -> &'static str {
    "Hello friend!"
}

#[rocket::main]
async fn main() {
    let config = Config::create_and_validate();
    rocket::build().mount("/", routes![index, api_interactions]).launch().await.expect("Failed to launch rocket :(");
}
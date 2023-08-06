mod headers;
mod config;
mod authorized_data;

#[macro_use] extern crate rocket;

use std::str::{from_utf8, Utf8Error};
use rocket::figment::util::vec_tuple_map::deserialize;
use serde::Deserialize;
use serenity::model::prelude::Interaction;
use crate::authorized_data::AuthorizedRequest;
use crate::config::Config;

#[post("/api/interactions", format= "application/json", data = "<message>")]
fn api_interactions(message: AuthorizedRequest) -> Result<&'static str, ()> {
    let message_string = match from_utf8(message.data.as_slice()) {
        Ok(val) => {val}
        Err(err) => {return Err(())}
    };
    let interaction: Interaction = serde_json::from_str(message_string).unwrap();
    println!("Interaction: {:?}", interaction);
    Ok("")
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
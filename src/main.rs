mod headers;
mod config;

#[macro_use] extern crate rocket;

use crate::config::Config;

#[post("/api/interactions")]
fn api_interactions() -> &'static str {
    "{\"type\":1}"
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
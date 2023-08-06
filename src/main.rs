mod headers;
mod config;
mod authorized_data;

#[macro_use] extern crate rocket;

use crate::authorized_data::AuthorizedRequest;
use crate::config::Config;

#[post("/api/interactions", format= "application/json", data = "<message>")]
fn api_interactions(message: AuthorizedRequest) -> Result<&'static str, ()> {
    Ok("{\"type\":1}")
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
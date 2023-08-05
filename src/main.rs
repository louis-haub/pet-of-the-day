#[macro_use] extern crate rocket;


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
    rocket::build().mount("/", routes![index, api_interactions]).launch().await;
}
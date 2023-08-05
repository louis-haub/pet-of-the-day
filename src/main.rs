#[macro_use] extern crate rocket;


#[post("/")]
fn base() -> &'static str {
    "{\"type\":1}"
}

#[get("/")]
fn index() -> &'static str {
    "Hello friend!"
}

#[rocket::main]
async fn main() {
    rocket::build().mount("/", routes![index]).launch().await;
}
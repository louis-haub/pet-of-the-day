#[macro_use] extern crate rocket;


#[post("/")]
fn index() -> &'static str {
    "{\"type\":1}"
}

#[rocket::main]
async fn main() {
    rocket::build().mount("/", routes![index]).launch().await;
}
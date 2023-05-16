// rocket 크레이트를 사용할 수 있도록 포함하는 것
#[macro_use] extern crate rocket;

use rocket::serde::json::{Value, json};

#[get("/")]
fn hello() -> Value {
   json!({
       "status": "ok",
       "message": "Hello, world!"
   })
}

#[rocket::main]
async fn main() {
    rocket::build()
    .mount("/", routes![hello])
    .launch()
    .await
    .unwrap();
}

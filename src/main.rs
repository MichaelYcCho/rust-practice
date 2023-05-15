// rocket 크레이트를 사용할 수 있도록 포함하는 것
#[macro_use] extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() {
    rocket::build()
    .mount("/", routes![hello])
    .launch()
    .await
    .unwrap();
}

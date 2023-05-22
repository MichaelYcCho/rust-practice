// rocket 크레이트를 사용할 수 있도록 포함하는 것
#[macro_use]
extern crate rocket;

mod auth;
use rocket::response::status;
use rocket::serde::json::{json, Value};
use rocket_sync_db_pools::database;

#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

#[get("/rustaceans")]
fn get_rustaceans(_auth: auth::BasicAuth, db: DbConn) -> Value {
    json!([{ "id": 1, "name": "John Doe" }, { "id": 2, "name": "John Doe again" }])
}

// 기본적으로 <id>가 매개변수명이다. fn의 매개인자와 일치해야 한다.
// #은 attribute를 의미한다.
#[get("/rustaceans/<id>")]
fn view_rustacean(id: i32, _auth: auth::BasicAuth) -> Value {
    json!({"id": id, "name": "John Doe", "email": "john@doe.com"})
}
#[post("/rustaceans", format = "json")]
fn create_rustacean(_auth: auth::BasicAuth) -> Value {
    json!({"id": 3, "name": "John Doe", "email": "john@doe.com"})
}
#[put("/rustaceans/<id>", format = "json")]
fn update_rustacean(id: i32, _auth: auth::BasicAuth) -> Value {
    json!({"id": id, "name": "John Doe", "email": "john@doe.com"})
}

// mock 데이터에선 id를 사용하지않아 경고가 뜨므로 _id로 명시한다
#[delete("/rustaceans/<_id>")]
fn delete_rustacean(_id: i32, _auth: auth::BasicAuth) -> status::NoContent {
    status::NoContent
}

#[catch(401)]
fn unauthorized() -> Value {
    json!("Invalid/Missing authorization")
}

#[catch(404)]
fn not_found() -> Value {
    json!({"status": "error", "reason": "Resource was not found."})
}

#[rocket::main]
async fn main() {
    rocket::build()
        .mount(
            "/",
            routes![
                get_rustaceans,
                view_rustacean,
                create_rustacean,
                update_rustacean,
                delete_rustacean
            ],
        )
        .register("/", catchers![not_found, unauthorized])
        .launch()
        .await
        .unwrap();
}

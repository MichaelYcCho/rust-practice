// rocket 크레이트를 사용할 수 있도록 포함하는 것
#[macro_use]
extern crate rocket;

mod auth;
mod models;
mod schema;

use auth::BasicAuth;
use diesel::prelude::*;
use models::{NewRustacean, Rustacean};
use rocket::response::status;
use rocket::serde::json::{json, Json, Value};
use rocket_sync_db_pools::database;
use schema::rustaceans;

#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

#[get("/rustaceans")]
// db.run()은 async 함수이므로 async fn으로 선언한다.
async fn get_rustaceans(_auth: BasicAuth, db: DbConn) -> Value {
    // 이 상태에서의 db는 connection이 아닌 pool이다. pool에서 연결을 하기 위해 선 run()을 사용한다
    // c는 connection이다. 이를 통해 db에 접근할 수 있다.
    db.run(|c| {
        // json의 양을 모두 로드하면 Ram이 부족할지모르니 일단 1000개만 불러보자
        let rustaceans = rustaceans::table
            .order(rustaceans::id.desc())
            .limit(1000)
            .load::<Rustacean>(c)
            .expect("DB error");
        json!(rustaceans)
    })
    .await
}

// 기본적으로 <id>가 매개변수명이다. fn의 매개인자와 일치해야 한다.
// #은 attribute를 의미한다.
#[get("/rustaceans/<id>")]
fn view_rustacean(id: i32, _auth: auth::BasicAuth) -> Value {
    json!({"id": id, "name": "John Doe", "email": "john@doe.com"})
}
#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(
    _auth: BasicAuth,
    db: DbConn,
    new_rustacean: Json<NewRustacean>,
) -> Value {
    db.run(|c| {
        let result = diesel::insert_into(rustaceans::table)
            // into_inner은 기본적으로 unwrap()을 호출한다.
            .values(new_rustacean.into_inner())
            .execute(c)
            .expect("DB error when inserting");
        json!(result)
    })
    .await
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
        .attach(DbConn::fairing())
        .launch()
        .await
        .unwrap();
}

/*
curl 127.0.0.1:8000/rustaceans -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==' -d '{"name": "John Doe", "email": "foo@bar.com"}' -H 'Content-Type: application/json'

 */

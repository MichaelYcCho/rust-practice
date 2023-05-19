// rocket 크레이트를 사용할 수 있도록 포함하는 것
#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::status;
use rocket::serde::json::{json, Value};

pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

impl BasicAuth {
    // Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==' 형식의 헤더를 파싱한다.
    fn from_authorization_header(header: &str) -> Option<BasicAuth> {
        // 헤더를 공백을 기준으로 분리한 후 벡터로 변환한다.
        let split = header.split_ascii_whitespace().collect::<Vec<_>>();
        if split.len() != 2 || split[0] != "Basic" {
            return None;
        }

        if split[0] != "Basic" {
            return None;
        }

        Self::from_base64_encoded(split[1])
    }

    // Option<T>는 Rust에서 제공하는 열거형(enum)으로, None 또는 Some(T)라는 두 가지 값을 가질 수 있다.
    // Option<BasicAuth>인 경우, 이 함수가 성공적으로 BasicAuth를 생성하면 Some(BasicAuth)를 반환하고, 그렇지 않으면 None을 반환합니다.
    fn from_base64_encoded(base64_string: &str) -> Option<BasicAuth> {
        // ? 연산자는 값이 있을경우 Some을 반환하고, None일 경우 None을 반환한 후 함수를 종료한다.
        let decoded = base64::decode(base64_string).ok()?;
        let decoded_str = String::from_utf8(decoded).ok()?;
        let split = decoded_str.split(":").collect::<Vec<_>>();

        // If exactly username & password pair are present
        if split.len() != 2 {
            return None;
        }

        let (username, password) = (split[0].to_string(), split[1].to_string());

        // username과 password가 모두 성공적으로 추출되면, 이 함수는 Some에 BasicAuth 인스턴스를 포함하여 반환한다.
        Some(BasicAuth { username, password })
    }
}

// FromRequest 트레이트 : Rocket의 request 개체를 수락하고 결과를 반환
// BasicAuth의 수명이 Request의 수명과 연관되어 있다는 것을 러스트 컴파일러에게 알려주는 것, BasicAuth 인스턴스는 해당 Request가 유효한 동안만 유효.
#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");
        if let Some(auth_header) = auth_header {
            // 헤더가 존재하면 BasicAuth::from_authorization_header()를 호출하여 BasicAuth 인스턴스를 생성한다.
            if let Some(auth) = Self::from_authorization_header(auth_header) {
                // 성공적으로 BasicAuth 인스턴스를 생성하면 Outcome::Success를 반환한다.
                return Outcome::Success(auth);
            }
        }

        // 인증에 실패하면 Outcome::Failure를 반환한다.
        Outcome::Failure((Status::Unauthorized, ()))
    }
}

#[get("/rustaceans")]
fn get_rustaceans(_auth: BasicAuth) -> Value {
    json!([{ "id": 1, "name": "John Doe" }, { "id": 2, "name": "John Doe again" }])
}

// 기본적으로 <id>가 매개변수명이다. fn의 매개인자와 일치해야 한다.
// #은 attribute를 의미한다.
#[get("/rustaceans/<id>")]
fn view_rustacean(id: i32) -> Value {
    json!({"id": id, "name": "John Doe", "email": "john@doe.com"})
}
#[post("/rustaceans", format = "json")]
fn create_rustacean() -> Value {
    json!({"id": 3, "name": "John Doe", "email": "john@doe.com"})
}
#[put("/rustaceans/<id>", format = "json")]
fn update_rustacean(id: i32) -> Value {
    json!({"id": id, "name": "John Doe", "email": "john@doe.com"})
}

// mock 데이터에선 id를 사용하지않아 경고가 뜨므로 _id로 명시한다
#[delete("/rustaceans/<_id>")]
fn delete_rustacean(_id: i32) -> status::NoContent {
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

use diesel::prelude::*;
use diesel::{Insertable, Queryable, AsChangeset};
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::schema::rustaceans;

#[derive(Serialize, Queryable)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,  // String -> NaiveDateTime으로 변경
}

// 삽입 가능한 속성이 필요하며, Diesel attr을 통해 어떤 테이블에 삽입할지 명시
#[derive(Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = rustaceans)]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}
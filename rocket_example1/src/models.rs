use crate::schema::rustaceans;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Queryable)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

// 삽입 가능한 속성이 필요, 또한 Diesel attr을 통해 어떤 테이블에 삽입할지 명시 해줘야 한다.
#[derive(Deserialize, Insertable)]
#[diesel(table_name = rustaceans)]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}

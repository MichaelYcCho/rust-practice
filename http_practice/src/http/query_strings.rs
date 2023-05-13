use std::{collections::HashMap};

// a=1&b=2&c&d=&e===&d=7&d=abc

// Request 구조체의 수명 buf와 같은 수명을 가진다.
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

// 힙 할당 동적 Array = 백터(Vec) -> 정해지지 않은 수의 Element를 가질 수 있다.
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

use super::method::Method;
use std::convert::TryFrom;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl Request {
    // 성공한다면 Request(Self)를 실패한다면 Error(String)을 반환한다.
    fn from_byte_array(buf: &[u8]) -> Result<Self, String> {}
}

// TryFrom 키워드뒤에 for을 넣고 우리가 Trait를 구현하고자 하는 타입을 넣어준다.
impl TryFrom<&[u8]> for Request {
    type Error = String;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

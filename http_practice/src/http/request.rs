use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

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

// Display Trait은 문자열을 형식화할때 사용된다 .
impl Display for ParsError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        // write! 매크로는 f를 통해 전달된 포맷팅된 문자열을 출력한다.
        write!(f, "{}", self.message())
    }
}

pub enum ParsError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParsError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

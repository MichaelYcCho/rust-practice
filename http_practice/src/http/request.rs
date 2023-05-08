use super::method::Method;
use ::std::str;
use ::std::str::Utf8Error;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

// TryFrom 키워드뒤에 for을 넣고 우리가 Trait를 구현하고자 하는 타입을 넣어준다.
impl TryFrom<&[u8]> for Request {
    type Error = ParseError;


    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        // buffer에 있는 byte가 유효한 UTF-8인지 확인
        // 방법1 match 사용
        /*
        match str::from_utf8(buf){
            Ok(request) => {}
            Err(_) => return Err(ParseError::InvalidEncoding),

        }
        */

        // 방법2-1 match +  from_utf8 내부 로직사용
        /*
        match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
            Ok(request) => {}
            Err(e) => return Err(e),
        }
        */

        // 방법2-2 from_utf8 내부 로직사용
        // ? -> 오류면 오류를 반환하고, 정상이면 정상을 반환한다.
        //let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;
        // 여기서 From을 통해 Utf8Error를 만들면 더 간소화할 수 있다.
        let request = str::from_utf8(buf)?;

        unimplemented!()
    }
}


// GET /search?name=abc&sort=1 HTTP/1.1 로 들어온 request를 파싱한다.
// return의 첫번째는 원하는 단어이고, 두번째는 남은 문자열이다.
fn get_next_word(request: &str) -> Option<(&str, &str)>{
    // 공백을 찾을때 까지 문자열 반복
    let mut iter = request.chars();
    loop{
        // next에 문자열이 없으면 None을 반환한다.
        let item = iter.next();
        match item{
            Some(c) => {}
            None => break,
        }
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<Utf8Error> for ParseError {
    //utf-8을 파라미터로 받고 ParseError를 반환한다.
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

// Display Trait은 문자열을 형식화할때 사용된다 .
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        // write! 매크로는 f를 통해 전달된 포맷팅된 문자열을 출력한다.
        write!(f, "{}", self.message())
    }
}

// Display Trait은 문자열을 형식화할때 사용된다 .
impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        // write! 매크로는 f를 통해 전달된 포맷팅된 문자열을 출력한다.
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}

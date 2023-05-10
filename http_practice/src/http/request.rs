use super::method::{Method, MethodError};
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
        // 위의 request와 아래의 request는 다른 것으로 보아야한다. == 변수 shadowing (로컬변수 이름 재사용)
        let request = str::from_utf8(buf)?;
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?; // ok_or는 Option<T>를 Result<T, E>로 변환한다.
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        // Header 부분
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        // parse : 받은 타입을 문자열에서 다른 타입으로 변환, FromStr 트레이트를 구현되어 있어야한다
        let method: Method = method.parse()?;
        let mut query_string = None;

        // 방법1
        match path.find('?'){
            Some(i) => {
                // i는 ?의 인덱스, 따라서 i 이후인 i+1 부터 가져온다
                // 그리고 여기서 1은 1byte를 의미한다.
                query_string = Some(&path[i + 1..]);
                // path를 mut로 변환한 후, ? 이전까지의 문자열을 가져온다.
                path = &path[..i];
            }
            None => {}
        }
        
        // 방법2
        let q = path.find('?');
        // some == 값이 있을때만 동작
        if q.is_some(){
            // Option이 None이면 패닉에 빠지기 때문에 unwrap을 사용한다.
            let i = q.unwrap();
            query_string = Some(&path[i + 1..]);
            path = &path[..i];
        }


        // 방법 3
        if let Some(i) = path.find('?'){
            query_string = Some(&path[i + 1..]);
            path = &path[..i];
        }

        unimplemented!()
    }
}

// GET /search?name=abc&sort=1/r/n HTTP/1.1 로 들어온 request를 파싱한다.
// return의 첫번째는 원하는 단어이고, 두번째는 남은 문자열이다.
fn get_next_word(request: &str) -> Option<(&str, &str)> {
    // 공백을 찾을때 까지 문자열 반복
    for (i, c) in request.chars().enumerate() {
        // 스페이스가 나오면 현재 인덱스 i까지의 문자열을 반환하고, 남은 문자열을 반환한다.
        // 이 경우 공백이 1byte라는것을 알기때문에 i+1 이지만, 이모지나 다른 것일경우엔 이를 고려해야한다.
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
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

impl From<MethodError> for ParseError {
    //MethodError를 파라미터로 받아 ParseError 타입으로 반환한다
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
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

use std::fmt::{Display, Formatter, Result as FmtResult};

// 컴파일러는 enum의 내용을 copy하고 이것을 clone한 후에 캐스팅한다.
#[derive(Clone, Copy, Debug)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "OK",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        // write! 매크로는 f를 통해 전달된 포맷팅된 문자열을 출력한다, enum을 u16으로 캐스팅한다.
        // 캐스팅 : 데이터 타입을 다른 데이터 타입으로 변환하는 것
        // 이때 참조가 지시하고있는 대상을 캐스팅하므로 *를 사용한다.

        write!(f, "{}", *self as u16)
    }
}

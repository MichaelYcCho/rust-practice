/*
crate는 전체 프로젝트의 루트를 가리킨다. 여기선 http_practice를 가리킨다.
이를 통해 해당 프로젝트의 main.rs의 http나 server에 접근할 수 있다.
 */
use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr, // addr: addr 과 같은 의미
        }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        // 반복 될때마다 새로운 연결이 있는지 체크
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024]; // 1024 바이트 크기의 배열을 만든다. 임의의 버퍼를 만들어서 클라이언트로부터 읽어들인 데이터를 저장할 것이다.
                    match stream.read(&mut buffer) {
                        // 소켓에서 클라이언트가 보낸 모든 바이트를 읽는다.
                        Ok(_) => {
                            //  UTF-8로 인코딩된 바이트 슬라이스(&[u8])를 입력 받아, 이를 문자열(String)로 변환하는 작업을 수행
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            //01. 컴파일러가 바이트 슬라이스로 취급하게 as키워드를 사용함
                            //Request::try_from(&buffer as &[u8]);

                            //02. 전체 Array가 담긴 byte Slice를 생성함
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {}
                                Err(e) => println!("Failed to parse a request: {}", e),
                            }

                            //03. TryInto 트레이트를 사용함
                            //let res: &Result<Request, _> = &buffer[..].try_into();
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => {
                    println!("Failed to establish a connection: {}", e);
                }
            }
        }
    }
}

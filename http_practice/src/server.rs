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

        /*
        res.unwrap()을 호출하면 Result 타입의 res 변수에서 값을 추출하려고 시도합니다.
        만약 res가 Ok 상태라면, unwrap()은 성공적으로 값을 반환하고, 이 경우 (stream, addr)에 할당됩니다.
        그러나 res가 Err 상태라면, unwrap()은 패닉(panic)을 발생시키고 프로그램이 종료됩니다.
        이 코드에서는 if res.is_err() { continue; }를 통해 에러가 발생한 경우 다음 연결 시도로 넘어가도록 합니다.
        이렇게 함으로써 연결에 실패할 경우 프로그램이 중단되지 않고 계속 실행되도록 할 수 있습니다.
        그러므로 이 코드에서 res.unwrap()이 호출될 때는 이미 res가 Ok 상태임이 보장되므로 패닉 없이 값을 안전하게 추출할 수 있습니다.
        */
        loop {
            // 새로운 TCP연결이 수립될때까지(이 주소에 새 연결이 도착할 때 까지)) 호출스레드를 차단한다

            let res = listener.accept();
            if res.is_err() {
                continue;
            }

            // 연결 실패시 다음 연결을 시도해봐야하므로 unwrap()을 사용한다.
            let (streamm, addr) = res.unwrap();
        }
    }
}

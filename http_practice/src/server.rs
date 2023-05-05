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
            match listener.accept(){
                Ok((stream, addr)) => {
                    println!("New client: {:?}", addr);
                }
                Err(e) => {
                    println!("Failed to establish a connection: {}", e);
                }

            }
        }
    }
}

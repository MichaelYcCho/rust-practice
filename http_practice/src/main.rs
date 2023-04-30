fn main() {
    let string = String::from("127.0.0.1:8000");
    let string_slice = &string[10..];
    let string_borrow : &str = &string; // 문자열 전체를 가르키는,  문자열 슬라이스 타입이 된다 

    dbg!(&string);
    dbg!(string_slice);

    // let server = Server::new("127.0.0.1:8000");
    // server.run();
}

// struct Server {
//     addr: String,
// }

// impl Server {
//     fn new(addr: String) -> Self {
//         Self {
//             addr, // addr: addr 과 같은 의미
//         }
//     }

//     fn run(self) {}
// }

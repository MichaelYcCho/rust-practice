fn main() {
    let get = Method::GET("abcd".to_string());
    let delete = Method::DELETE(100);
    let post = Method::POST;
    let put = Method::PUT;


    
    let server = Server::new("127.0.0.1:8000".to_string());
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self {
            addr, // addr: addr 과 같은 의미
        }
    }

    fn run(self) {
        println!("Listening on {}", self.addr);
    }
}

struct Request {
    path: String,
    query_string: String,
    method: Method,
}

enum Method {
    GET(String),
    DELETE(u64),
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

fn main() {
    let server = server::Server::new("127.0.0.1:8000".to_string());
    server.run();
}

mod server {

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
        }
    }
}

mod http {
    mod request {
        struct Request {
            path: String,
            query_string: Option<String>,
            method: super::method::Method,
        }
    }

    mod method {
        pub enum Method {
            GET,
            DELETE,
            POST,
            PUT,
            HEAD,
            CONNECT,
            OPTIONS,
            TRACE,
            PATCH,
        }
    }
}

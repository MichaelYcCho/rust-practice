fn main() {
    let server = Server::new("127.0.0.1:8000");
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

    fn run(self) {}
}

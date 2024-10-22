use pigeon::Server;

#[tokio::main]
async fn main() {
    let mut server = Server::new("127.0.0.1:3000".into());
    server.run();
}
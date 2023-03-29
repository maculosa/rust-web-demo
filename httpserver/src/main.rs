mod server;
mod router;
mod handler;

use server::Server;

fn main() {
    let server = Server::new("localhost:3400");
    server.run();
}

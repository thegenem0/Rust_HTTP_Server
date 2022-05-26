#![allow(dead_code)]

mod http;
mod server;
mod web_handler;

use server::Server;
use std::env;
use web_handler::WebHandler;

fn main() {
    let def_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let pub_path = env::var("PUBLIC_PATH").unwrap_or(def_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebHandler::new(pub_path));
}

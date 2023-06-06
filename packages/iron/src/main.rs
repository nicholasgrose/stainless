#![forbid(unsafe_code)]
#![feature(error_generic_member_access, provide_any)]

use web::start_server;

mod database;
mod shared;
mod web;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    start_server("127.0.0.1:8080").await
}

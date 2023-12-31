use api::init;
use std::env;

mod api;
mod config;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    dotenvy::dotenv().ok();
    init().await
}

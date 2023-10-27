#![allow(unused)]
#[macro_use]
extern crate dotenvy_macro;
#[macro_use]
extern crate rust_i18n;

#[cfg(test)]
pub(crate) mod mocks;

mod actions;
mod data;
mod enums;
mod error;
mod http;
// mod logger;
mod models;
mod prelude;
mod sys;
mod utils;

use database::DatabaseManager;
use sys::config;
use log::Level;
use std::net::SocketAddr;

i18n!("lang", fallback = "en");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let router = crate::http::init().await?;
    let server_config = config().server();
    let server_address = SocketAddr::from(([127, 0, 0, 1], server_config.port()));
    let server = axum::Server::bind(&server_address)
        .serve(router.into_make_service());

    println!("Server listening at http://{}", &server_address);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
        std::process::abort();
    }

    Ok(())
}

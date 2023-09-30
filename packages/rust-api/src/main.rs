#![allow(unused)]
#[macro_use]
extern crate dotenvy_macro;
#[macro_use]
extern crate rust_i18n;

mod actions;
mod data;
mod enums;
mod http;
mod kore;
// mod logger;
mod models;
mod root;
mod sys;
mod types;
mod utils;

pub(crate) use http::context::Context;
pub(crate) use http::response::JsonResponse;
pub(crate) use kore::Result;
pub(crate) use root::errors::{Error, ErrorContext};
pub(crate) use root::std::Result as KoreResult;
pub(crate) use sys::{config, DatabaseManager};
pub(crate) use utils::__;
pub(crate) use axum_session::SessionPgSession as Session;

use log::Level;
use std::net::SocketAddr;

i18n!("lang", fallback = "en");

#[tokio::main]
async fn main() -> core::result::Result<(), Box<crate::types::DynError>> {
    match simple_logger::init_with_level(Level::Info) {
        Ok(_) => {}
        Err(_) => {
            println!("Error setting up logger");
        }
    }

    let router = crate::http::init().await?;
    let server_config = config().server();
    let server_address = SocketAddr::from(([127, 0, 0, 1], server_config.port()));
    let server = axum::Server::bind(&server_address).serve(router.into_make_service());

    println!("Server listening at http://{}", &server_address);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
        std::process::abort();
    }

    Ok(())
}

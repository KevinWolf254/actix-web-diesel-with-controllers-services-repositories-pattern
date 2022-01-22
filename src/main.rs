#[macro_use]
extern crate diesel;
extern crate dotenv;

use tracing::info;
use color_eyre::Result;
use crate::config::Config;
use actix_web::{App, HttpServer, middleware::Logger};
use diesel::{r2d2::{self, ConnectionManager}, PgConnection,};

mod config;
mod models;
mod schema;
mod services;
mod controllers;
mod repositories;
type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> Result<()> {

    let config = Config::from_env().expect("Server configuration");

    let pool = config.database_pool().await
        .expect("Database configuration.");

    info!("Starting server at http://{}:{}/", config.host, config.port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pool.clone())
            .configure(controllers::init_organisation_controller)
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await?;

    Ok(())
}

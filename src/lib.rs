#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::sync::{Mutex};
use diesel::{PgConnection, Connection};
use dotenv::dotenv;
use std::env;

pub mod config;
pub mod controllers;
pub mod repositories;
pub mod models;
pub mod schema;

pub struct AppState<> {
    pub connections: Mutex<u32>,
    // pub context: Arc<Database<'a>>,
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
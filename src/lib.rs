#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::{PgConnection, r2d2::{self, ConnectionManager}};

// pub mod config;
pub mod controllers;
pub mod services;
pub mod repositories;
pub mod models;
pub mod schema;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
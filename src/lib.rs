use std::sync::{Arc, Mutex};


pub mod config;
pub mod controllers;

pub struct AppState<> {
    pub connections: Mutex<u32>,
    // pub context: Arc<Database<'a>>,
}
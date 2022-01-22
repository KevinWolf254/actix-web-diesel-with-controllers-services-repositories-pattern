use color_eyre::Result;
use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};
use dotenv::dotenv;
use eyre::WrapErr;
use serde::Deserialize;
use tracing::info;
use tracing_subscriber::EnvFilter;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i32,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Result<Config> {
        dotenv().ok();

        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();

        info!("Loading configuration");

        let mut c = config::Config::new();
        c.merge(config::Environment::default())?;

        c.try_into()
            .context("Loading configuration from environment")
    }

    pub async fn database_pool(&self) -> Result<Pool> {
        info!("Creating database connection pool.");

        // create db connection pool
        let manager = ConnectionManager::<PgConnection>::new(&self.database_url);

        r2d2::Pool::builder()
            .build(manager)
            .context("Creating database connection pool.")
    }
}

use actix_web::{App, HttpServer};

use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use rent_management_system_api::{controllers, Pool};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .configure(controllers::init_organisation_controller)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

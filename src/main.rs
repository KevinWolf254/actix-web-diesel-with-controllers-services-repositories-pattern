use actix_web::{App, HttpServer};
use rent_management_system_api::{controllers};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = HttpServer::new(move || {
        App::new()

            .configure(controllers::init_organisation_controller)

    })
    .bind("127.0.0.1:8080")?;

    app.run().await
}

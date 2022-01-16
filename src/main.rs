use actix_web::{web, App, HttpServer};
use rent_management_system_api::config::Config;
// use sqlx_user_crud::dao::Database;
use rent_management_system_api::{controllers, AppState};
use std::sync::{Arc, Mutex};

// #[macro_use]
// extern crate diesel;

// mod organisations;
// mod schema;

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // HttpServer::new(|| App::new().service(hello))
    //     .bind("127.0.0.1:8080")?
    //     .run()
    //     .await
    // let app_state = web::Data::new(AppState {
    //     connections: Mutex::new(0),
    //     context: Arc::new(db_context),
    // });

    let app = HttpServer::new(move || {
        App::new()
            // .app_data(app_state.clone())
            .configure(controllers::init_user_controller)
    })
    // .bind(config.get_app_url())?;
    .bind("127.0.0.1:8080")?;
    
    app.run().await
}

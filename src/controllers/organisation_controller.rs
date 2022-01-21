use actix_web::{post, web, HttpResponse, Responder};
use crate::establish_connection;
use crate::{models::organisation::CreateOrganisation, repositories::organisation_repository::create_organisation};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
}

#[post("/organisation")]
async fn create(req_body: web::Json<CreateOrganisation>) -> impl Responder {
    let conn = establish_connection();
    let created_organisation = create_organisation(&conn, &req_body);
    HttpResponse::Ok().json(created_organisation)
}
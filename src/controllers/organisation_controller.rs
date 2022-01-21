use crate::Pool;
use actix_web::{post, web, HttpResponse, Error};
use crate::models::organisation::{CreateOrganisationRequest};
use crate::services::organisation_service::create_organisation;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
}

#[post("/organisation")]
pub async fn create(db: web::Data<Pool>, request: web::Json<CreateOrganisationRequest>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || create_organisation(db, request))
    .await
    .map(|organisation| HttpResponse::Created().json(organisation))
    .map_err(|_| HttpResponse::InternalServerError())?)
}
use crate::repositories::organisation_repository;
use actix_web::web::{Data, Json};
use crate::Pool;
use chrono::Local;
use crate::models::organisation::{Organisation, CreateOrganisation, CreateOrganisationRequest};

pub fn create_organisation(db: Data<Pool>, request: Json<CreateOrganisationRequest>) -> Result<Organisation, diesel::result::Error> {

    let new_organisation = CreateOrganisation {
        name: request.name.as_str(),
        pin_number: request.pin_number.as_str(),
        description: match &request.description {
            None => None,
            Some(desc) => Some(desc.as_str())
        },
        vat_number: match &request.vat_number {
            None => None,
            Some(vat) => Some(vat.as_str())
        },
        created: Local::now().naive_local(),
    };

    organisation_repository::create_organisation(db, new_organisation)
}
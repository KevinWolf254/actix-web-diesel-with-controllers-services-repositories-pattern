use crate::{repositories::organisation_repository, models::organisation::UpdateOrganisation};
use actix_web::web::{Data, Json};
use crate::Pool;
use chrono::Local;
use crate::models::organisation::{Organisation, CreateOrganisation, CreateOrganisationRequest, UpdateOrganisationRequest};


pub fn find_by_id(database_pool: &Data<Pool>, organisation_id: i32) -> Result<Option<Organisation>, diesel::result::Error> {
    let optional_organisation = organisation_repository::find_by_id(database_pool, organisation_id).unwrap();
    if let Some(organisation) = optional_organisation {    
        Ok(Some(organisation))
    } else {
        Err(diesel::result::Error::NotFound)
    }
}

pub fn create(database_pool: &Data<Pool>, create_request: Json<CreateOrganisationRequest>) -> Result<Organisation, diesel::result::Error> {
    let request = create_request.into_inner();

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

    organisation_repository::save(database_pool, new_organisation)
}

pub fn update(database_pool: &Data<Pool>, update_request: Json<UpdateOrganisationRequest>) -> Result<Organisation, diesel::result::Error> {
    let request = update_request.into_inner();

    let optional_organisation = organisation_repository::find_by_id(database_pool, request.id).unwrap();
   
    if let Some(organisation) = optional_organisation {
    
        let organisation = UpdateOrganisation {
            id: organisation.id,
            description: match request.description {
                None => None,
                Some(desc) => Some(desc)
            },
            vat_number: match request.vat_number {
                None => None,
                Some(vat) => Some(vat)
            },
            updated: Local::now().naive_local(),
        };
    
        organisation_repository::update(database_pool, organisation)
    } else {
        Err(diesel::result::Error::NotFound)
    }
}

pub fn delete_by_id(database_pool: &Data<Pool>, organisation_id: i32) -> Result<usize, diesel::result::Error> {
    let optional_organisation = organisation_repository::find_by_id(database_pool, organisation_id).unwrap();
        
    if let Some(organisation) = optional_organisation {
        organisation_repository::delete_by_id(database_pool, organisation.id)
    } else {
        Err(diesel::result::Error::NotFound)
    }
}
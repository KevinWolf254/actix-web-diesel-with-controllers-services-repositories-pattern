use crate::Pool;
use actix_web::error::BlockingError;
use actix_web::{post, put, get, delete, web, HttpResponse, Error};
use crate::models::organisation::{CreateOrganisationRequest, UpdateOrganisationRequest};
use crate::services::organisation_service::{create, update, find_by_id, delete_by_id};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create_organisation);
    cfg.service(update_organisation);
    cfg.service(find_organisation_by_id);
    cfg.service(delete_organisation_by_id);
}

#[post("/organisation")]
pub async fn create_organisation(database_pool: web::Data<Pool>, request: web::Json<CreateOrganisationRequest>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || create(&database_pool, request))
    .await
    .map(|organisation| HttpResponse::Created().json(organisation))
    .map_err(|_| HttpResponse::InternalServerError())?)
}

#[put("/organisation")]
pub async fn update_organisation(database_pool: web::Data<Pool>, request: web::Json<UpdateOrganisationRequest>) -> Result<HttpResponse, Error> {
   
    let block = web::block(move || update(&database_pool, request));

    let response = match block.await {
        Ok(organisation) => HttpResponse::Ok().json(organisation),
        Err(BlockingError::Error(diesel::result::Error::NotFound)) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    };

    Ok(response)
}

#[get("/organisation/{id}")]
async fn find_organisation_by_id( database_pool: web::Data<Pool>, organisation_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let organisation_id = organisation_id.into_inner();

    // use web::block to offload blocking Diesel code without blocking server thread
    let block = web::block(move || find_by_id(&database_pool, organisation_id));
    
    let response = match block.await {
        Ok(optional_organisation) => {
            if let Some(organisation) = optional_organisation {
                HttpResponse::Ok().json(organisation)
            } else {
                HttpResponse::NotFound()
                    .body(format!("No organisation found with id: {}", organisation_id))
            }
        },
        Err(BlockingError::Error(diesel::result::Error::NotFound)) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    };

    Ok(response)
    // let optional_organisation = block
        
    //     .await
    //     .map_err(|e| {
    //         eprintln!("{}", e);
    //         HttpResponse::InternalServerError().finish()
    //     })?;

    // if let Some(organisation) = optional_organisation {
    //     Ok(HttpResponse::Ok().json(organisation))
    // } else {
    //     let res = HttpResponse::NotFound()
    //         .body(format!("No organisation found with id: {}", organisation_id));
    //     Ok(res)
    // }
}

#[delete("/organisation/{id}")]
async fn delete_organisation_by_id(database_pool: web::Data<Pool>, organisation_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let organisation_id = organisation_id.into_inner();

    match delete_by_id(&database_pool, organisation_id) {
        Ok(_) => {
            Ok(HttpResponse::Ok().finish())
        },
        Err(_) => {
            Ok(HttpResponse::NotFound().finish())
        }
    }
}
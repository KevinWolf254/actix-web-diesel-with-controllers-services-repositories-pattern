use actix_web::web::Data;
use crate::Pool;
use diesel::{insert_into, RunQueryDsl};
use crate::schema::organisations::dsl::organisations;
use crate::models::organisation::{Organisation, CreateOrganisation};

pub fn create_organisation(db: Data<Pool>, new_organisation: CreateOrganisation) -> Result<Organisation, diesel::result::Error> {
    let conn = db.get().unwrap();

    let result = insert_into(organisations).values(&new_organisation).get_result(&conn)?;

    Ok(result)
}
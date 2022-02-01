use actix_web::web::Data;
use crate::{Pool, models::organisation::UpdateOrganisation};
use crate::schema::organisations::dsl::*;
use diesel::{insert_into, RunQueryDsl, query_dsl::methods::FilterDsl, ExpressionMethods, OptionalExtension};
use crate::models::organisation::{Organisation, CreateOrganisation};

pub fn find_by_id(database_pool: &Data<Pool>, organisation_id: i32) -> Result<Option<Organisation>, diesel::result::Error> {
    let conn = database_pool.get().unwrap();
    
    let result = organisations
        .filter(id.eq(organisation_id))
        .first::<Organisation>(&conn)
        .optional()?;

    Ok(result)
}

pub fn save(database_pool: &Data<Pool>, new_organisation: CreateOrganisation) -> Result<Organisation, diesel::result::Error> {
    let conn = database_pool.get().unwrap();

    let result = insert_into(organisations).values(new_organisation).get_result(&conn)?;

    Ok(result)
}

pub fn update(database_pool: &Data<Pool>, updated_organisation: UpdateOrganisation) -> Result<Organisation, diesel::result::Error> {
    use crate::diesel::QueryDsl;
    
    let conn = database_pool.get().unwrap();
    
    diesel::update(crate::schema::organisations::table.find(updated_organisation.id))
        .set(&updated_organisation)
        .get_result(&conn)
}


pub fn delete_by_id(database_pool: &Data<Pool>, organisation_id: i32) -> Result<usize, diesel::result::Error> {
    use crate::diesel::QueryDsl;
    
    let conn = database_pool.get().unwrap();

    let source = crate::schema::organisations::table.find(organisation_id);
    diesel::delete(source)
        .execute(&conn)
}
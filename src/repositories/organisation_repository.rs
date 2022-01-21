// use diesel::PgConnection;
use diesel::prelude::*;

use crate::models::organisation::{Organisation, CreateOrganisation};
use crate::schema::organisations;

pub fn create_organisation(conn: &PgConnection, organisation: &CreateOrganisation) -> Organisation {
    diesel::insert_into(organisations::table)
        .values(organisation)
        .get_result::<Organisation>(conn)
        .expect("Error saving new organisation")
}
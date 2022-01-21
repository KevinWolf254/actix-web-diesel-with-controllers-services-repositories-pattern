use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::schema::organisations;

#[derive(Debug, Clone, Serialize, Queryable)]
pub struct Organisation {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub vat_number: Option<String>,
    pub pin_number: String,
    pub created: NaiveDateTime,
    pub updated: Option<NaiveDateTime>
}


#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "organisations"]
pub struct CreateOrganisation<'a> {
    pub name: &'a str,
    pub pin_number: &'a str,
    pub description: Option<&'a str>,
    pub vat_number: Option<&'a str>,
    pub created: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrganisationRequest {
    pub name: String,
    pub pin_number: String,
    pub description: Option<String>,
    pub vat_number: Option<String>,
}
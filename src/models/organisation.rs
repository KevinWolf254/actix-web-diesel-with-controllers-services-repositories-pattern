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


#[derive(Debug, Clone, Deserialize, Insertable)]
#[table_name = "organisations"]
pub struct CreateOrganisation {
    pub name: String,
    pub description: Option<String>,
    pub vat_number: Option<String>,
    pub pin_number: String,
    pub created: NaiveDateTime 
}
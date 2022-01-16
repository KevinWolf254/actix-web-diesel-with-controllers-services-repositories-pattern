use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "organisations"]
pub struct Organisation {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub vat_number: Option<String>,
    pub pin_number: String,
    pub created: DateTime<Utc>,
    pub updated: Option<DateTime<Utc>>,
}

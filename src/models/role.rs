use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Role {
    pub id: Option<Thing>,
    pub name: String,
    pub createdAt: Option<DateTime<Local>>
}

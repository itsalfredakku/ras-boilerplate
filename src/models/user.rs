use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: Option<Thing>,
    pub name: String,
    pub email: String,
    pub phone: Option<bool>,
    pub role: Option<Thing>,
    pub createdAt: Option<DateTime<Local>>,
    pub updatedAt: Option<DateTime<Local>>,
}

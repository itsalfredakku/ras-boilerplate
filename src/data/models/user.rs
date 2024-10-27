use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use crate::data::models::role::Role;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: Option<Thing>,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub role: Role,
    pub created_at: Option<DateTime<Local>>,
    pub updated_at: Option<DateTime<Local>>,
}

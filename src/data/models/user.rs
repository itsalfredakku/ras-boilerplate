use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: Option<Thing>,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub role_id: Option<Thing>,
    pub created_at: Option<DateTime<Local>>,
    pub updated_at: Option<DateTime<Local>>,
}

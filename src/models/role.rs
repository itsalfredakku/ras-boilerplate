use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Role {
    pub id: Option<Thing>,
    pub name: String,
    pub created_at: Option<DateTime<Local>>,
}

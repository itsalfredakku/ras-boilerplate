use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Role {
    pub id: Option<Thing>,
    pub name: String,
    pub users: Option<Vec<Thing>>,
    pub created_at: Option<DateTime<Local>>,
    pub updated_at: Option<DateTime<Local>>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateRole {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateRole {
    pub name: String,
}

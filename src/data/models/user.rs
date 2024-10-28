use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: Option<Thing>,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub role: Option<Thing>,
    pub created_at: Option<DateTime<Local>>,
    pub updated_at: Option<DateTime<Local>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub role: Option<Thing>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateUser {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub role: Option<Thing>,
}

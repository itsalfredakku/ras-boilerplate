use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Todo {
    pub id: Option<Thing>,
    pub title: String,
    pub content: String,
    // Use Option<bool> correctly here, assuming you want to track completion status.
    pub completed: Option<bool>,
    pub created_at: Option<DateTime<Local>>,
    pub updated_at: Option<DateTime<Local>>,
}

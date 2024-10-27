use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Todo {
    pub id: Option<Thing>,
    pub title: String,
    pub content: Option<String>,
    pub completed: Option<bool>,
    pub created_at: Option<DateTime<Local>>,
    pub updated_at: Option<DateTime<Local>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateTodo {
    pub title: String,
    pub content: Option<String>,
    pub completed: Option<bool>,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub content: Option<String>,
    pub completed: Option<bool>,
}
use std::sync::Arc;
use axum::{response::IntoResponse, Extension, Json};

use crate::{
    data::repositories::todo_repository::TodoRepository, models::todo::Todo,
};
use crate::db::Database;

pub async fn get_all_todos_query(Extension(db): Extension<Arc<Database>>) -> impl IntoResponse {
    let repository = TodoRepository::new(db);

    let mut todos: Vec<Todo> = Vec::new();
    if let Ok(result) = repository.get_all().await {
        todos = result;
    }

    let json_response = serde_json::json!({
        "status": "success".to_string(),
        "results": todos.len(),
        "todos": todos,
    });

    Json(json_response)
}

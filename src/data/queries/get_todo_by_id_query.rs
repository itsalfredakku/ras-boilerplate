use std::sync::Arc;
use axum::{extract::Path, response::IntoResponse, Json, http::StatusCode, Extension};

use crate::data::repositories::todo_repository::TodoRepository;
use crate::db::Database;

pub async fn get_todo_by_id_query(
    Extension(db): Extension<Arc<Database>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let repository = TodoRepository::new(db);
    let id = id.to_string();

    let todo = repository.get_by_id(id).await;

    return Ok((StatusCode::OK, Json(todo)));
}

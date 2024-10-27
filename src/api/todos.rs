use axum::{
    routing::{get, post},
    Router,
};

use crate::data::{commands::{create_todo_command::create_todo_command, update_todo_command::update_todo_command, delete_todo_command::delete_todo_command}, queries::{get_all_todos_query::get_all_todos_query, get_todo_by_id_query::get_todo_by_id_query}};
pub fn todos_router() -> Router {
    Router::new()
        .route(
            "/",
            post(create_todo_command)
            .get(get_all_todos_query),
        )
        .route(
            "/:id",
            get(get_todo_by_id_query)
            .put(update_todo_command)
            .delete(delete_todo_command)
        )
}

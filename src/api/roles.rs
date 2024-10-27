use axum::{
    routing::{get},
    Router,
};

use crate::data::{queries::{get_all_todos_query::get_all_todos_query}};
pub fn roles_router() -> Router {
    Router::new()
        .route(
            "/",
            get(get_all_todos_query),
        )
}

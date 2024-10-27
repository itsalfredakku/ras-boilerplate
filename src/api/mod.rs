pub mod todos;
pub mod users;
pub mod roles;
pub mod healthcheck;

pub mod models;

pub mod api_router {
    use axum::Router;
    use axum::routing::get;
    use crate::api::{roles::roles_router, todos::todos_router, users::users_router};
    use crate::api::healthcheck::healthcheck_handler;

    pub fn api_router() -> Router {
        Router::new()
            .route("/healthcheck", get(healthcheck_handler))
            .nest("/todos", todos_router())
            .nest("/users", users_router())
            .nest("/roles", roles_router())
    }
}
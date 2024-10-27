pub mod healthcheck_handler;
pub mod roles_router;
pub mod todos_router;
pub mod users_router;

pub mod api_router {
    use crate::routers::healthcheck_handler::healthcheck_handler;
    use crate::routers::{roles_router::roles_router, todos_router::todos_router, users_router::users_router};
    use axum::routing::get;
    use axum::Router;

    pub fn api_router() -> Router {
        Router::new()
            .route("/healthcheck", get(healthcheck_handler))
            .nest("/todos", todos_router::router())
            .nest("/users", users_router::router())
            .nest("/roles", roles_router::router())
    }
}

pub mod healthcheck;
pub mod roles;
pub mod todos;
pub mod users;

pub mod api_router {
    use crate::routers::healthcheck::healthcheck_handler;
    use crate::routers::{roles::roles_router, todos::todos_router, users::users_router};
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

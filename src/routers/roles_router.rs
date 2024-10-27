pub mod roles_router {
    use crate::repositories::roles_repository::RolesRepository;
    use crate::database::Database;
    use crate::models::role::Role;
    use axum::extract::Path;
    use axum::http::StatusCode;
    use axum::{
        response::IntoResponse,
        routing::{get, post},
        Extension, Json, Router,
    };
    use std::sync::Arc;

    pub fn router() -> Router {
        Router::new()
            .route("/", post(create_role).get(get_all_roles))
            .route(
                "/:id",
                get(get_role_by_id).put(update_role).delete(delete_role),
            )
            .route("/name/:name", get(get_role_by_name))
    }

    pub async fn get_all_roles(Extension(db): Extension<Arc<Database>>) -> impl IntoResponse {
        let repository = RolesRepository::new(db);
        let roles = repository.get_all().await.unwrap_or_default();
        Json(serde_json::json!({
            "status": "success",
            "count": roles.len(),
            "roles": roles
        }))
    }

    pub async fn get_role_by_id(
        Extension(db): Extension<Arc<Database>>,
        Path(id): Path<String>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
        let repository = RolesRepository::new(db);
        match repository.get_by_id(id.clone()).await {
            Ok(role) => Ok((StatusCode::OK, Json(role))),
            Err(_) => Err((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "status": "error",
                    "message": format!("Role with ID: {} not found", id)
                })),
            )),
        }
    }

    pub async fn get_role_by_name(
        Extension(db): Extension<Arc<Database>>,
        Path(name): Path<String>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
        let repository = RolesRepository::new(db);
        match repository.get_by_name(name.clone()).await {
            Ok(role) => Ok((StatusCode::OK, Json(role))),
            Err(_) => Err((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "status": "error",
                    "message": format!("Role with name: {} not found", name)
                })),
            )),
        }
    }

    pub async fn create_role(
        Extension(db): Extension<Arc<Database>>,
        Json(body): Json<Role>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
        let repository = RolesRepository::new(db);
        if let Ok(role) = repository.get_by_name(body.name.clone()).await {
            let json_response = serde_json::json!({
                "status": "error",
                "message": "Role already exists",
                "repositories": role,
            });
            return Err((StatusCode::BAD_REQUEST, Json(json_response)));
        }
        match repository.create(body.clone()).await {
            Ok(role) => Ok((
                StatusCode::CREATED,
                Json(serde_json::json!({
                    "status": "success",
                    "repositories": role[0].to_owned()
                })),
            )),
            Err(_) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "status": "error",
                    "message": "Failed to create role"
                })),
            )),
        }
    }

    pub async fn update_role(
        Extension(db): Extension<Arc<Database>>,
        Path(id): Path<String>,
        Json(body): Json<Role>,
    ) -> impl IntoResponse {
        let repository = RolesRepository::new(db);
        match repository.update(id.clone(), body.clone()).await {
            Ok(role) => (
                StatusCode::OK,
                Json(serde_json::json!({
                    "status": "success",
                    "repositories": role
                })),
            ),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "status": "error",
                    "message": "Failed to update role"
                })),
            ),
        }
    }

    pub async fn delete_role(
        Extension(db): Extension<Arc<Database>>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        let repository = RolesRepository::new(db);
        match repository.delete(id.clone()).await {
            Ok(_) => (
                StatusCode::NO_CONTENT,
                Json(serde_json::json!({
                    "status": "success",
                    "message": "Role deleted successfully"
                })),
            ),
            Err(_) => (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "status": "error",
                    "message": format!("Role with ID: {} not found", id)
                })),
            ),
        }
    }
}

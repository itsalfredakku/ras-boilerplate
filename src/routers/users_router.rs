pub mod users_router {
    use std::sync::Arc;
    use axum::{routing::{get, post}, Extension, Json, Router, response::IntoResponse};
    use axum::extract::Path;
    use axum::http::StatusCode;
    use crate::data::contexts::user_repository::UserRepository;
    use crate::db::Database;
    use crate::models::user::User;

    pub fn router() -> Router {
        Router::new()
            .route("/", post(create_user).get(get_all_users))
            .route("/:id", get(get_user_by_id).put(update_user).delete(delete_user))
    }

    pub async fn get_all_users(Extension(db): Extension<Arc<Database>>) -> impl IntoResponse {
        let repository = UserRepository::new(db);
        let users = repository.get_all().await.unwrap_or_default();
        Json(serde_json::json!({
            "status": "success",
            "results": users.len(),
            "users": users
        }))
    }

    pub async fn get_user_by_id(
        Extension(db): Extension<Arc<Database>>,
        Path(id): Path<String>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
        let repository = UserRepository::new(db);
        match repository.get_by_id(id.clone()).await {
            Ok(user) => Ok((StatusCode::OK, Json(user))),
            Err(_) => Err((StatusCode::NOT_FOUND, Json(serde_json::json!({
                "status": "error",
                "message": format!("User with ID: {} not found", id)
            }))))
        }
    }

    pub async fn create_user(
        Extension(db): Extension<Arc<Database>>,
        Json(body): Json<User>,
    ) ->  Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
        let repository = UserRepository::new(db);
        match repository.create(body.clone()).await {
            Ok(user) => Ok((StatusCode::CREATED, Json(serde_json::json!({
                "status": "success",
                "data": user
            })))),
            Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "status": "error",
                "message": "Failed to create user"
            }))))
        }
    }

    pub async fn update_user(
        Extension(db): Extension<Arc<Database>>,
        Path(id): Path<String>,
        Json(body): Json<User>,
    ) -> impl IntoResponse {
        let repository = UserRepository::new(db);
        match repository.update(id.clone(), body.clone()).await {
            Ok(user) => (StatusCode::OK, Json(serde_json::json!({
                "status": "success",
                "data": user
            }))),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "status": "error",
                "message": "Failed to update user"
            })))
        }
    }

    pub async fn delete_user(
        Extension(db): Extension<Arc<Database>>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        let repository = UserRepository::new(db);
        match repository.delete(id.clone()).await {
            Ok(_) => (StatusCode::NO_CONTENT, Json(serde_json::json!({
                "status": "success",
                "message": "User deleted successfully"
            }))),
            Err(_) => (StatusCode::NOT_FOUND, Json(serde_json::json!({
                "status": "error",
                "message": format!("User with ID: {} not found", id)
            })))
        }
    }
}

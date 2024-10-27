pub mod users_router {
    use crate::data::models::user::{CreateUser, UpdateUser, User};
    use crate::data::repositories::users_repository::UsersRepository;
    use crate::db::Database;
    use axum::extract::Path;
    use axum::http::StatusCode;
    use axum::{
        response::IntoResponse,
        routing::{get, post},
        Extension, Json, Router,
    };
    use chrono::Local;
    use std::sync::Arc;

    pub fn router() -> Router {
        Router::new()
            .route("/", post(create_user).get(get_all_users))
            .route(
                "/:id",
                get(get_user_by_id).put(update_user).delete(delete_user),
            )
            .route("/email/:email", get(get_user_by_email))
            .route("/phone/:phone", get(get_user_by_phone))
    }

    pub async fn get_all_users(Extension(db): Extension<Arc<Database>>) -> impl IntoResponse {
        let repository = UsersRepository::new(db);
        let users = repository.get_all().await.unwrap_or_default();
        Json(serde_json::json!({
            "status": "success",
            "count": users.len(),
            "users": users
        }))
    }

    pub async fn get_user_by_id(
        Extension(db): Extension<Arc<Database>>,
        Path(id): Path<String>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
        let repository = UsersRepository::new(db);
        match repository.get_by_id(id.clone()).await {
            Ok(user) => Ok((StatusCode::OK, Json(user))),
            Err(_) => Err((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "status": "error",
                    "message": format!("User with ID: {} not found", id)
                })),
            )),
        }
    }

    pub async fn get_user_by_email(
        Extension(db): Extension<Arc<Database>>,
        Path(email): Path<String>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
        let repository = UsersRepository::new(db);
        match repository.get_by_email(email.clone()).await {
            Ok(user) => Ok((StatusCode::OK, Json(user))),
            Err(_) => Err((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "status": "error",
                    "message": format!("User with email: {} not found", email)
                })),
            )),
        }
    }

    pub async fn get_user_by_phone(
        Extension(db): Extension<Arc<Database>>,
        Path(phone): Path<String>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
        let repository = UsersRepository::new(db);
        match repository.get_by_phone(phone.clone()).await {
            Ok(user) => Ok((StatusCode::OK, Json(user))),
            Err(_) => Err((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "status": "error",
                    "message": format!("User with phone: {} not found", phone)
                })),
            )),
        }
    }

    pub async fn create_user(
        Extension(db): Extension<Arc<Database>>,
        Json(body): Json<CreateUser>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
        let repository = UsersRepository::new(db);
        
        match repository.get_by_email(body.email.clone()).await {
            Ok(user) => {
                let json_response = serde_json::json!({
                    "status": "error",
                    "message": "User already exists",
                    "user": user,
                });
                return Err((StatusCode::BAD_REQUEST, Json(json_response)));
            }
            Err(_) => {
                let datetime = Local::now();
                let user = User {
                    id: None,
                    name: body.name.clone(),
                    email: body.email.clone(),
                    phone: body.phone.clone(),
                    role: body.role.clone(),
                    created_at: Some(datetime),
                    updated_at: Some(datetime),
                };
                match repository.create(user.clone()).await {
                    Ok(user_response) => Ok((
                        StatusCode::CREATED,
                        Json(serde_json::json!({
                            "status": "success",
                            "user": user_response
                        })),
                    )),
                    Err(_) => Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({
                            "status": "error",
                            "message": "Failed to create user"
                        })),
                    )),
                }
            }
        }
        
        // match repository.create(body.clone()).await {
        //     Ok(user) => Ok((
        //         StatusCode::CREATED,
        //         Json(serde_json::json!({
        //             "status": "success",
        //             "user": user
        //         })),
        //     )),
        //     Err(_) => Err((
        //         StatusCode::INTERNAL_SERVER_ERROR,
        //         Json(serde_json::json!({
        //             "status": "error",
        //             "message": "Failed to create user"
        //         })),
        //     )),
        // }
    }


    pub async fn update_user(
        Extension(db): Extension<Arc<Database>>,
        Path(id): Path<String>,
        Json(body): Json<UpdateUser>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
        let repository = UsersRepository::new(db);

        match repository.get_by_id(id.clone()).await {
            Ok(mut user) => {
                let datetime = Local::now();
                user.name = body.name.clone();
                user.email = body.email.clone();
                user.phone = body.phone.clone();
                user.role = body.role.clone();
                user.updated_at = Some(datetime);

                match repository.update(id.clone(), user.clone()).await {
                    Ok(user_response) => Ok((
                        StatusCode::OK,
                        Json(serde_json::json!({
                            "status": "success",
                            "user": user_response
                        })),
                    )),
                    Err(_) => Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({
                            "status": "error",
                            "message": "Failed to update todo"
                        })),
                    )),
                }
            }
            Err(_) => Err((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "status": "error",
                    "message": format!("Todo with ID: {} not found", id)
                })),
            )),
        }
        // match repository.get_by_id(id.clone()).await {
        //     Ok(mut user) => {
        //         user.name = body.name.clone();
        //         user.email = body.email.clone();
        //         user.phone = body.phone.clone();
        //         user.role_id = body.role_id.clone();
        //         let user = repository.update(id, user).await.unwrap();
        //
        //         Json(serde_json::json!({
        //             "status": "success",
        //             "user": user
        //         }))
        //     }
        //     Err(_) => Json(serde_json::json!({
        //         "status": "error",
        //         "message": format!("User with ID: {} not found", id)
        //     })),
        // }
    }

    pub async fn delete_user(
        Extension(db): Extension<Arc<Database>>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        let repository = UsersRepository::new(db);
        match repository.delete(id.clone()).await {
            Ok(_) => (
                StatusCode::NO_CONTENT,
                Json(serde_json::json!({
                    "status": "success",
                    "message": "User deleted successfully"
                })),
            ),
            Err(_) => (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "status": "error",
                    "message": format!("User with ID: {} not found", id)
                })),
            ),
        }
    }
}

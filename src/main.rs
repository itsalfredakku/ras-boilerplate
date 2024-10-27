use rss_boilerplate::{db::Database};
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use tower_http::cors::CorsLayer;
use std::net::SocketAddr;
extern crate dotenv;
use dotenv::dotenv;
use std::env;
use axum::{Extension, Router};
use rss_boilerplate::api::api_router::api_router;

#[tokio::main]
async fn main() {
    // Initialize the environment variables
    dotenv().ok();

    // Load the environment variables
    let host = env::var("HOST").unwrap_or("0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or("8080".to_string());
    let allowed_origins = env::var("ALLOWED_ORIGINS").unwrap_or("http://localhost:3000".to_string());

    // Connect to the database
    let db = Database::init().await.expect("Failed to connect to the database");
    println!("🚀 Database connected successfully");

    // Setup the CORS layer
    let cors = CorsLayer::new()
        .allow_origin(allowed_origins.parse::<HeaderValue>().expect("Invalid ALLOWED_ORIGINS header value"))
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    // Create the router
    let app = Router::new()
        .nest("/api", api_router())
        .layer(cors)
        .layer(Extension(db));

    // Start the server
    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Invalid host or port");

    println!("🚀 Server starting at {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind address");
    axum::serve(listener, app).await.expect("Server failed to start");
}

use std::sync::Arc;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::{Root},
    Result, Surreal,
};
use std::env;

#[derive(Debug, Clone)]
pub struct Database {
    pub client: Surreal<Client>,
    pub namespace: String,
    pub database: String,
}

impl Database {
    pub async fn init() -> Result<Arc<Self>> {
        // Load required environment variables
        let surreal_address = env::var("SURREAL_ADDRESS").expect("SURREAL_ADDRESS environment variable is not set");
        let surreal_user = env::var("SURREAL_USER").expect("SURREAL_USER environment variable is not set");
        let surreal_password = env::var("SURREAL_PASSWORD").expect("SURREAL_PASSWORD environment variable is not set");
        let surreal_namespace = env::var("SURREAL_NAMESPACE").expect("SURREAL_NAMESPACE environment variable is not set");
        let surreal_database = env::var("SURREAL_DATABASE").expect("SURREAL_DATABASE environment variable is not set");

        // Establish database connection
        let client = Surreal::new::<Ws>(&surreal_address).await?;

        // Authenticate with provided credentials
        client.signin(Root {
            username: &surreal_user,
            password: &surreal_password,
        }).await?;

        // Set namespace and database context
        client.use_ns(&surreal_namespace).use_db(&surreal_database).await?;

        Ok(Arc::new(Self {
            client,
            namespace: surreal_namespace,
            database: surreal_database,
        }))
    }
}

// Lazy initialization for a globally accessible database instance
// pub static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);
//
// pub async fn connect_db() -> Result<()> {
//     // Load the environment variables
//     let surreal_address = env::var("SURREAL_ADDRESS").expect("SURREAL_ADDRESS environment variable is not set");
//     let surreal_user = env::var("SURREAL_USER").expect("SURREAL_USER environment variable is not set");
//     let surreal_password = env::var("SURREAL_PASSWORD").expect("SURREAL_PASSWORD environment variable is not set");
//     let surreal_ns = env::var("SURREAL_NAMESPACE").expect("SURREAL_NAMESPACE environment variable is not set");
//     let surreal_database = env::var("SURREAL_DATABASE").expect("SURREAL_DATABASE environment variable is not set");
//
//     // Connect to the database using the global instance
//     DB.connect::<Ws>(&surreal_address).await?;
//     DB.signin(Root {
//         username: &surreal_user,
//         password: &surreal_password,
//     }).await?;
//     DB.use_ns(&surreal_ns).use_db(&surreal_database).await?;
//     
//     Ok(())
// }

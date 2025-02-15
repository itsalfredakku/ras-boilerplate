use crate::db::Database;
use crate::data::models::user::User;
use std::sync::Arc;
use chrono::Local;
use surrealdb::err::Error::Thrown;
use surrealdb::Error;

pub struct UsersRepository {
    db: Arc<Database>,
    table: String,
}

impl UsersRepository {
    pub fn new(db: Arc<Database>) -> Self {
        UsersRepository {
            db,
            table: String::from("user"),
        }
    }

    pub async fn get_all(&self) -> Result<Vec<User>, Error> {
        let records = self.db.client.select(&self.table).await?;
        Ok(records)
    }

    pub async fn get_by_id(&self, id: String) -> Result<User, Error> {
        if let Some(record) = self.db.client.select((&self.table, id.clone())).await? {
            return Ok(record);
        }

        Err(Error::Db(Thrown(format!("User with id {} not found", id))))
    }

    pub async fn get_by_email(&self, email: String) -> Result<User, Error> {
        if let Some(record) = self
            .db
            .client
            .query("SELECT * FROM user WHERE email = $email")
            .bind(("email", email.clone()))
            .await?
            .take(0)?
        {
            return Ok(record);
        }

        Err(Error::Db(Thrown(format!(
            "User with email {} not found",
            email
        ))))
    }

    pub async fn get_by_phone(&self, phone: String) -> Result<User, Error> {
        if let Some(record) = self
            .db
            .client
            .query("SELECT * FROM user WHERE phone = $phone")
            .bind(("phone", phone.clone()))
            .await?
            .take(0)?
        {
            return Ok(record);
        }

        Err(Error::Db(Thrown(format!(
            "User with phone {} not found",
            phone
        ))))
    }

    pub async fn create(&self, mut user: User) -> Result<User, Error> {
        user.created_at = Some(Local::now());
        let record = self
            .db
            .client
            .create(&self.table)
            .content(user)
            .await?
            .ok_or_else(|| Error::Db(Thrown("Failed to create user".to_string())))?;
        Ok(record)
    }

    pub async fn update(&self, id: String, mut user: User) -> Result<User, Error> {
        user.updated_at = Some(Local::now());
        let record = self
            .db
            .client
            .update((&self.table, id.clone()))
            .content(user)
            .await?
            .ok_or(Error::Db(Thrown(format!("User with id {} not found", id))))?;
        Ok(record)
    }

    pub async fn delete(&self, id: String) -> Result<User, Error> {
        let record = self
            .db
            .client
            .delete((&self.table, id.clone()))
            .await?
            .ok_or(Error::Db(Thrown(format!("User with id {} not found", id))))?;
        Ok(record)
    }
}

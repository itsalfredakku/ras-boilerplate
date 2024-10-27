use crate::database::Database;
use crate::models::todo::Todo;
use std::sync::Arc;
use surrealdb::{error::Db::Thrown, Error};

pub struct TodoRepository {
    db: Arc<Database>,
    table: String,
}

impl TodoRepository {
    pub fn new(db: Arc<Database>) -> Self {
        TodoRepository {
            db,
            table: String::from("todo"),
        }
    }

    pub async fn get_all(&self) -> Result<Vec<Todo>, Error> {
        let records = self.db.client.select(&self.table).await?;
        Ok(records)
    }

    pub async fn get_by_id(&self, id: String) -> Result<Todo, Error> {
        if let Some(record) = self.db.client.select((&self.table, id.clone())).await? {
            return Ok(record);
        }

        Err(Error::Db(Thrown(format!("Todo with id {} not found", id))))
    }

    pub async fn get_by_title(&self, title: String) -> Result<Todo, Error> {
        if let Some(record) = self
            .db
            .client
            .query("SELECT * FROM todo WHERE title = $title")
            .bind(("title", title.clone()))
            .await?
            .take(0)?
        {
            return Ok(record);
        }

        Err(Error::Db(Thrown(format!(
            "Todo with title {} not found",
            title
        ))))
    }

    pub async fn create(&self, content: Todo) -> Result<Vec<Todo>, Error> {
        let record = self
            .db
            .client
            .create(&self.table)
            .content(content)
            .await?
            .ok_or_else(|| Error::Db(Thrown("Failed to create todo".to_string())))?;
        Ok(record)
    }

    pub async fn update(&self, id: String, content: Todo) -> Result<Todo, Error> {
        let record = self
            .db
            .client
            .update((&self.table, id.clone()))
            .content(content)
            .await?
            .ok_or(Error::Db(Thrown(format!("Todo with id {} not found", id))))?;
        Ok(record)
    }

    pub async fn delete(&self, id: String) -> Result<Todo, Error> {
        let result = self
            .db
            .client
            .delete((&self.table, id.clone()))
            .await?
            .ok_or(Error::Db(Thrown(format!("Todo with id {} not found", id))))?;
        Ok(result)
    }
}

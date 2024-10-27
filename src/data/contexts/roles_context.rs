use crate::database::Database;
use crate::models::role::Role;
use std::sync::Arc;
use surrealdb::err::Error::Thrown;
use surrealdb::Error;

pub struct RoleRepository {
    db: Arc<Database>,
    table: String,
}

impl RoleRepository {
    pub fn new(db: Arc<Database>) -> Self {
        RoleRepository {
            db,
            table: String::from("role"),
        }
    }

    pub async fn get_all(&self) -> Result<Vec<Role>, Error> {
        let records = self.db.client.select(&self.table).await?;
        Ok(records)
    }

    pub async fn get_by_id(&self, id: String) -> Result<Role, Error> {
        if let Some(record) = self.db.client.select((&self.table, id.clone())).await? {
            return Ok(record);
        }

        Err(Error::Db(Thrown(format!("Role with id {} not found", id))))
    }

    pub async fn get_by_name(&self, name: String) -> Result<Role, Error> {
        if let Some(record) = self
            .db
            .client
            .query("SELECT * FROM role WHERE name = $name")
            .bind(("name", name.clone()))
            .await?
            .take(0)?
        {
            return Ok(record);
        }

        Err(Error::Db(Thrown(format!(
            "Role with name {} not found",
            name
        ))))
    }

    pub async fn create(&self, content: Role) -> Result<Vec<Role>, Error> {
        let record = self
            .db
            .client
            .create(&self.table)
            .content(content)
            .await?
            .ok_or_else(|| Error::Db(Thrown("Failed to create role".to_string())))?;
        Ok(record)
    }

    pub async fn update(&self, id: String, content: Role) -> Result<Role, Error> {
        let record = self
            .db
            .client
            .update((&self.table, id.clone()))
            .content(content)
            .await?
            .ok_or(Error::Db(Thrown(format!("Role with id {} not found", id))))?;
        Ok(record)
    }

    pub async fn delete(&self, id: String) -> Result<Role, Error> {
        let record = self
            .db
            .client
            .delete((&self.table, id.clone()))
            .await?
            .ok_or(Error::Db(Thrown(format!("Role with id {} not found", id))))?;
        Ok(record)
    }
}

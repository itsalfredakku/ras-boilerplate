use std::sync::Arc;
use crate::data::repositories::role_repository::RoleRepository;
use crate::data::repositories::todo_repository::TodoRepository;
use crate::data::repositories::user_repository::UserRepository;
use crate::db::Database;

pub struct DataContext {
    db: Arc<Database>,
}

impl DataContext {
    pub fn new(db: Arc<Database>) -> Self {
        DataContext {
            db,
        }
    }

    pub fn todos(&self) -> TodoRepository {
        TodoRepository::new(self.db.clone())
    }
    
    pub fn users(&self) -> UserRepository {
        UserRepository::new(self.db.clone())
    }
    
    pub fn roles(&self) -> RoleRepository {
        RoleRepository::new(self.db.clone())
    }
}
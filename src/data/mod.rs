pub mod contexts;


#[derive(Clone, Debug)]
pub struct DataContext {
    db: std::sync::Arc<crate::db::Database>,
}

impl DataContext {
    pub fn new(db: std::sync::Arc<crate::db::Database>) -> Self {
        DataContext {
            db,
        }
    }

    pub fn todos(&self) -> crate::data::contexts::todos_context::TodoRepository {
        crate::data::contexts::todos_context::TodoRepository::new(self.db.clone())
    }

    pub fn users(&self) -> crate::data::contexts::user_repository::UserRepository {
        crate::data::contexts::user_repository::UserRepository::new(self.db.clone())
    }

    pub fn roles(&self) -> crate::data::contexts::roles_context::RoleRepository {
        crate::data::contexts::roles_context::RoleRepository::new(self.db.clone())
    }
}
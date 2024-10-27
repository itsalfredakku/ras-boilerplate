use crate::data;

#[derive(Clone, Debug)]
pub struct DataContext {
    db: std::sync::Arc<crate::db::Database>,
}

impl DataContext {
    pub fn new(db: std::sync::Arc<crate::db::Database>) -> Self {
        DataContext { db }
    }

    pub fn todos(&self) -> data::repositories::todos_repository::TodosRepository {
        data::repositories::todos_repository::TodosRepository::new(self.db.clone())
    }

    pub fn users(&self) -> data::repositories::users_repository::UsersRepository {
        data::repositories::users_repository::UsersRepository::new(self.db.clone())
    }

    pub fn roles(&self) -> data::repositories::roles_repository::RolesRepository {
        data::repositories::roles_repository::RolesRepository::new(self.db.clone())
    }
}

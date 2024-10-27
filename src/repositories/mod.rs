pub mod roles_repository;
pub mod todos_repository;
pub mod users_repository;

#[derive(Clone, Debug)]
pub struct DataContext {
    db: std::sync::Arc<crate::database::Database>,
}

impl DataContext {
    pub fn new(db: std::sync::Arc<crate::database::Database>) -> Self {
        DataContext { db }
    }

    pub fn todos(&self) -> todos_repository::TodosRepository {
        todos_repository::TodosRepository::new(self.db.clone())
    }

    pub fn users(&self) -> users_repository::UsersRepository {
        users_repository::UsersRepository::new(self.db.clone())
    }

    pub fn roles(&self) -> roles_repository::RolesRepository {
        roles_repository::RolesRepository::new(self.db.clone())
    }
}

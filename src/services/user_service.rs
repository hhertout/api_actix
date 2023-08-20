use sea_orm::DatabaseConnection;

use crate::config::db::DB;
use entity::user::Entity as User;
use sea_orm::EntityTrait;

pub struct UserService {
    pub db: DatabaseConnection,
}

impl UserService {
    pub fn new() -> Self {
        Self {
            db: unsafe { DB.get_connection() },
        }
    }

    pub async fn find_all(&self) -> Vec<entity::user::Model> {
        let result = User::find().all(unsafe { &DB.get_connection() }).await;
        match result {
            Ok(users) => users,
            Err(e) => {
                panic!("❌ Failed to insert record: {:?}", e);
            }
        }
    }
}

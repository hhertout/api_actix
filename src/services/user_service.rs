use sea_orm::DatabaseConnection;

use crate::config::db::DB;
use entity::user::Entity as User;
use entity::*;
use sea_orm::EntityTrait;
use sea_orm::Set;
use sea_orm::ActiveModelTrait;

pub struct UserService {
    pub db: DatabaseConnection,
}

impl UserService {
    pub fn new() -> Self {
        Self {
            db: unsafe { DB.get_connection() },
        }
    }

    pub async fn find_all(&self) -> Result<Vec<entity::user::Model>, sea_orm::DbErr> {
        User::find().all(unsafe { &DB.get_connection() }).await
    }

    pub async fn create_user(&self, user: entity::user::Model) -> Result<entity::user::ActiveModel, sea_orm::DbErr> {
        // TODO - Check if user already exists
        // TODO - Hash password
        user::ActiveModel {
            firstname: Set(user.firstname),
            lastname: Set(user.lastname),
            email: Set(user.email),
            password: Set(user.password),
            ..Default::default()
        }.save(&self.db).await
    }
}

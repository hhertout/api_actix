use sea_orm::DatabaseConnection;

use entity::user::Entity as User;
use entity::*;
use sea_orm::ActiveModelTrait;
use sea_orm::EntityTrait;
use sea_orm::Set;

pub struct UserService {}

impl UserService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn find_all(
        &self,
        db: &DatabaseConnection,
    ) -> Result<Vec<entity::user::Model>, sea_orm::DbErr> {
        User::find().all(db).await
    }

    pub async fn create_user(
        &self,
        db: &DatabaseConnection,
        user: entity::user::Model,
    ) -> Result<entity::user::ActiveModel, sea_orm::DbErr> {
        // TODO - Check if user already exists
        // TODO - Hash password
        user::ActiveModel {
            firstname: Set(user.firstname),
            lastname: Set(user.lastname),
            email: Set(user.email),
            password: Set(user.password),
            ..Default::default()
        }
        .save(db)
        .await
    }
}

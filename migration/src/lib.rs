pub use sea_orm_migration::prelude::*;

mod m20230815_175227_user;
mod m20230817_192951_post;
mod m20230817_194958_post;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230815_175227_user::Migration),
            Box::new(m20230817_192951_post::Migration),
            Box::new(m20230817_194958_post::Migration),
        ]
    }
}

use sea_orm::{Database, DatabaseConnection};

#[derive(Clone)]
pub struct Db {
    db: Option<DatabaseConnection>,
}

impl Db {
    pub const fn new() -> Db {
        Db { db: None }
    }

    // Connect to database
    pub async fn connect(&mut self) {
        let database_url = dotenvy::var("DATABASE_URL").unwrap();
        let database = Database::connect(database_url).await;
        match database {
            Ok(_) => {
                let db = database.unwrap();
                println!("✅ Successfully connected to database");
                self.db = Some(db);
            }
            Err(e) => {
                panic!("❌ Failed to connect to database: {}", e);
            }
        }
    }

    pub fn get_connection(&self) -> DatabaseConnection {
        self.db.clone().unwrap()
    }
}

pub static mut DB: Db = Db::new();

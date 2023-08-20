pub mod user_api;

use actix_web::web::{Data, ServiceConfig};
use actix_web::{middleware::Logger, App};
use actix_web::{HttpServer, Result};
use migration::Migrator;
use migration::MigratorTrait;
use sea_orm::DatabaseConnection;

use crate::api::user_api::*;
use crate::config::db::Db;

#[derive(Debug, Clone)]
struct AppState {
    db: DatabaseConnection,
}

// All routes are define here
fn router(cfg: &mut ServiceConfig) {
    cfg.service(get_users);
    cfg.service(create_user);
}

pub async fn init() -> Result<(), std::io::Error> {
    let uri = dotenvy::var("SERVER_URI").unwrap_or_else(|_| "localhost".to_string());
    let port = dotenvy::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());

    let mut db = Db::new();
    db.connect().await;
    let connection = db.get_connection();

    Migrator::up(&connection, None).await.unwrap();

    let state = AppState { db: connection };

    println!("🚀 Server currently running at http://{}:{}/", uri, port);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new(
                "Request => %a \"%r\"; status => %s; time => %Dms",
            ))
            .app_data(Data::new(state.clone()))
            .configure(router)
    })
    .bind((uri.as_str(), port.as_str().parse().unwrap()))?
    .run()
    .await
}

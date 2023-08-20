use std::env;
use actix_web::middleware::Logger;
use actix_web::{web, Responder, Result, get};
use actix_web::{App, HttpServer};
use sea_orm::Database;
use entity::user::Entity as User;
use sea_orm::EntityTrait;

#[get("/users")]
async fn index() -> Result<impl Responder> {
    let database = dotenvy::var("DATABASE_URL").unwrap();
    let database = Database::connect(database.as_str()).await;
    let db = match database {
        Ok(_) => {
            println!("✅ Successfully connected to database");
            database.unwrap()
        }
        Err(e) => {
            panic!("❌ Failed to connect to database: {}", e);
        }
    };
    let result = User::find().all(&db).await;
    let users = match result {
        Ok(users) => {
            users
        }
        Err(e) => {
            panic!("❌ Failed to insert record: {:?}", e);
        }
    };
    Ok(web::Json(users))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    dotenvy::dotenv().ok();
    let uri = dotenvy::var("SERVER_URI").unwrap_or_else(|_| "localhost".to_string());
    let port = dotenvy::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());

    println!("🚀 Server currently running at http://{}:{}/", uri, port);
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new(
                "Request => %a \"%r\"; status => %s; time => %Dms",
            ))
            .service(index)
    })
    .bind((uri.as_str(), port.as_str().parse().unwrap()))?
    .run()
    .await
}

pub mod user_api;

use crate::services::user_service::UserService;
use actix_web::{get, web, Responder, Result, HttpServer};
use actix_web::{middleware::Logger, App};

#[get("/users")]
async fn index() -> Result<impl Responder> {
    let users = UserService::new().find_all().await;
    Ok(web::Json(users))
}

pub async fn init() -> Result<(), std::io::Error> {
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

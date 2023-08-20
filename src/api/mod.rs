pub mod user_api;

use actix_web::{ Result, HttpServer};
use actix_web::{middleware::Logger, App};

use crate::api::user_api::*;

pub async fn init() -> Result<(), std::io::Error> {
    let uri = dotenvy::var("SERVER_URI").unwrap_or_else(|_| "localhost".to_string());
    let port = dotenvy::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());

    println!("🚀 Server currently running at http://{}:{}/", uri, port);
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new(
                "Request => %a \"%r\"; status => %s; time => %Dms",
            ))
            .service(get_users)
    })
    .bind((uri.as_str(), port.as_str().parse().unwrap()))?
    .run()
    .await
}

use crate::services::user_service::UserService;
use actix_web::{get, web, Responder, Result, post};

#[get("/user/all")]
async fn get_users() -> Result<impl Responder> {
    let users = UserService::new().find_all().await;
    Ok(web::Json(users))
}
use crate::services::user_service::UserService;
use actix_web::{get, web, Responder, Result, post};
use entity::user::Model as User;

#[get("/user/all")]
async fn get_users() -> Result<impl Responder> {
    let result = UserService::new().find_all().await;
    match result {
        Ok(users) => Ok(web::Json(users)),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

#[post("/user/create")]
async fn create_user(user: web::Json<User>) -> Result<impl Responder> {
    let result = UserService::new().create_user(user.into_inner()).await;
    match result {
        Ok(_) => Ok(web::Json("User created successfully")),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}
use crate::api::AppState;
use crate::services::user_service::UserService;
use actix_web::{get, post, web, Responder, Result};
use entity::user::Model as User;

#[get("/user/all")]
async fn get_users(data: web::Data<AppState>) -> Result<impl Responder> {
    let result = UserService::new().find_all(&data.db).await;
    match result {
        Ok(users) => Ok(web::Json(users)),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

#[post("/user/create")]
async fn create_user(data: web::Data<AppState>, user: web::Json<User>) -> Result<impl Responder> {
    let result = UserService::new()
        .create_user(&data.db, user.into_inner())
        .await;
    match result {
        Ok(_) => Ok(web::Json("User created successfully")),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

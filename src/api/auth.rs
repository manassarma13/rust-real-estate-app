use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct UserCredentials {
    username: String,
    password: String,
}

#[post("/register")]
async fn register_user(credentials: web::Json<UserCredentials>) -> impl Responder {
    HttpResponse::Ok().body("User registered")
}

#[post("/login")]
async fn login_user(credentials: web::Json<UserCredentials>) -> impl Responder {
    HttpResponse::Ok().body("User logged in")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register_user).service(login_user);
}

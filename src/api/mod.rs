mod auth;
mod properties;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(auth::init_routes)
            .configure(properties::init_routes)
    );
}

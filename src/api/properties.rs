use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Property {
    id: u32,
    title: String,
    price: f64,
}

#[get("/properties")]
async fn get_properties() -> impl Responder {
    // Logic to get all properties
    HttpResponse::Ok().json(vec![
        Property { id: 1, title: "Property 1".to_string(), price: 500000.0 },
        Property { id: 2, title: "Property 2".to_string(), price: 750000.0 },
    ])
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_properties);
}

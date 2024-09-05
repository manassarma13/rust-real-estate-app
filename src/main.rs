use actix_web::{App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use std::env;
use dotenv::dotenv;
use tokio::signal;

mod api;
mod db;
mod handlers;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            eprintln!("DATABASE_URL must be set");
            std::process::exit(1);
        }
    };

    let manager: ConnectionManager<PgConnection> = ConnectionManager::<PgConnection>::new(database_url);
    let pool = match r2d2::Pool::builder().build(manager) {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to create pool: {}", e);
            std::process::exit(1);
        }
    };

    let server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .configure(api::init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run();

    tokio::select! {
        _ = signal::ctrl_c() => {
            println!("Shutting down gracefully...");
        }
        _ = server => {}
    }

    Ok(())
}

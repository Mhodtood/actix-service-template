mod routes;

use self::routes::health;
use actix_cors::Cors;
use actix_web::{App, HttpServer};

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse::<u16>()
        .unwrap();
    HttpServer::new(|| App::new().service(health).wrap(Cors::default()))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}

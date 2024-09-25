mod routes;

use self::routes::health;
use actix_cors::Cors;
use actix_web::{App, HttpServer};

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(health).wrap(Cors::default()))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

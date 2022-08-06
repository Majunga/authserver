#[macro_use]
mod config;

use actix_web::{App, HttpServer, Responder, HttpResponse, get};
use actix_web::middleware::Logger;
use color_eyre::Result;
use tracing::{info, instrument};

use self::config::Config;

#[get("/healthprobe")]
async fn healthprobe() -> impl Responder {
    HttpResponse::Ok().body("All good")
}

#[actix_rt::main]
#[instrument]
async fn main() -> Result<()> {
    let config = Config::new()
        .expect("Server config");

    info!("Starting server at http://{}:{}/", config.host, config.port);

    HttpServer::new(|| {
        App::new()
         .service(healthprobe)
         .wrap(Logger::default())
    })
        .bind(format!("{}:{}", config.host, config.port))
        .expect("Server bind")
        .run()
        .await?;
    
    Ok(())
}

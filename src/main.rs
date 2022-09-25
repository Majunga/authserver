use actix_web::{Responder, HttpResponse, get, HttpServer};
use oauth2::basic::BasicClient;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

struct AppState {
    oauth: BasicClient,
}

#[get("/healthprobe")]
async fn healthprobe() -> impl Responder {
    HttpResponse::Ok().body("All good")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    builder.set_private_key_file("key.pem", SslFiletype::PEM)?;
    builder.set_certificate_chain_file("cert.pem")?;

    HttpServer::new(|| {
        actix_web::App::new()
            .service(healthprobe)
    })
    .bind_openssl("localhost:8080", builder)?
    .run()
    .await
}

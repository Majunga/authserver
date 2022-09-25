mod auth_clients;

use actix_web::{Responder, HttpResponse, get, HttpServer, web::Data};
use oauth2::basic::BasicClient;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

struct AppState {
    oauth: BasicClient,
}

#[get("/healthprobe")]
async fn healthprobe() -> impl Responder {
    HttpResponse::Ok().body("All good")
}

fn build_ssl() -> SslAcceptorBuilder {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    return builder;
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {


    HttpServer::new(|| {
        let oauth = auth_clients::google_client::build_google_auth();

        actix_web::App::new()
            .app_data(Data::new(AppState { oauth }))
            .service(healthprobe)
    })
    .bind_openssl("localhost:4433", build_ssl())?
    .run()
    .await
}

use super::controller;
use actix_web::{web, Scope};

pub fn routes() -> Scope {
    web::scope("/authentication")
        .service(controller::login)
        .service(controller::logout)
        .service(controller::auth)
}
extern crate serde_derive;

use actix_session::Session;
use actix_web::{HttpResponse, web, http::header, Responder};
use oauth2::{PkceCodeChallenge, CsrfToken, Scope, AuthorizationCode};
use serde::Deserialize;

use crate::AppState;

#[derive(Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
    scope: String,
}

#[actix_web::get("/login")]
pub async fn login(data: web::Data<AppState>) -> impl Responder {
  let (pkce_code_challenge, _pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

  let(authorize_url, _csrf_state) = &data
    .oauth
    .authorize_url(CsrfToken::new_random)
    .add_scope(Scope::new("https://www.googleapis.com/auth/userinfo.email".to_string()))
    .set_pkce_challenge(pkce_code_challenge)
    .url();

  HttpResponse::Found()
    .append_header((header::LOCATION, authorize_url.to_string()))
    .finish()
}

#[actix_web::get("/logout")]
pub async fn logout(session: Session) -> impl Responder {
    session.remove("login");
    HttpResponse::Ok().body("Logged out")
}

#[actix_web::get("/auth")]
pub async fn auth(session: Session, data: web::Data<AppState>, params: web::Query<AuthRequest>) -> impl Responder {
  let code = AuthorizationCode::new(params.code.clone());
  let state = CsrfToken::new(params.state.clone());
  let _scope = params.scope.clone();

  let token = &data.oauth.exchange_code(code);

  session.insert("login", true).unwrap();

  let html = format!(
    r#"
    <html>
      <head>
        <title>Actix OAuth2 Example</title>
      </head>
      <body>
        <h1>Actix OAuth2 Example</h1>
        <p>Logged in!</p>
        Google returned the following state:
        <pre>{}</pre>
        Google returned the following token:
        <pre>{:?}</pre>
        <p><a href="authentication/logout">Logout</a></p>
      </body>
    </html>
    "#,
    state.secret(),
    token
  );

  HttpResponse::Ok().body(html)
}
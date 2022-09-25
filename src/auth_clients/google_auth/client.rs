use std::env;
use oauth2::{basic::BasicClient, ClientId, ClientSecret, AuthUrl, TokenUrl, RedirectUrl};

pub fn build_google_auth() -> BasicClient {
  let client_id = ClientId::new(
      env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set"),
  );

  let client_secret = ClientSecret::new(
      env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET must be set"),
  );

  let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
  .expect("Invalid authorization endpoint URL");
let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v4/token".to_string())
  .expect("Invalid token endpoint URL");

  let oauth = BasicClient::new(
      client_id,
      Some(client_secret),
      auth_url,
      Some(token_url),
  ).set_redirect_uri(RedirectUrl::new("https://127.0.0.1:4433/auth".to_string()).expect("Invalid redirect URL")); 

  return oauth;
}
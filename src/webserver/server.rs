#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use actix_web::HttpRequest;
use actix_web::cookie::Cookie;
use actix_web::{get, App, HttpResponse, HttpServer, Responder, web, http::header};
use openssl::ssl::{SslAcceptor, SslMethod};
use rand::Rng;
use rand::distributions::Alphanumeric;
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use tokio_tungstenite::tungstenite::Message;


// Local imports
use crate::websocket::moderation::Blacklist;
use super::routes::oauth::auth::auth_token;

pub struct AppState {
    pub tx : async_channel::Sender<Message>,
    pub blacklist: Arc<Blacklist>,
    pub req_client: reqwest::Client,
    pub client_id: String,
    pub client_secret: String,
    pub pg_pool: Pool<Postgres>,
}


pub async fn start_server( tx: async_channel::Sender<Message>, blacklist: Arc<Blacklist>, pool: Pool<Postgres>) {
    let req_client = reqwest::Client::new();
    let mut builder =SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file("key.pem", openssl::ssl::SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    let app_state = web::Data::new(AppState {
        tx,
        blacklist,
        req_client,
        client_id: env::var("CLIENT_ID").expect("No Client ID"),
        client_secret: env::var("CLIENT_SECRET").expect("No Client Secret"),
        pg_pool: pool,
    });
    HttpServer::new(move|| {
        App::new().app_data(app_state.clone()).service(index).service(auth_token).service(authenticate)
    })
    .bind_openssl("127.0.0.1:8080",builder)
    .unwrap()
    .run()
    .await
    .unwrap();

}


#[get("/authenticate")]
async fn authenticate(_data: web::Data<AppState>) -> impl Responder {
    let state = generate_state();

    let cookie = Cookie::build("oauth_state", &state).http_only(true).finish();
    // TODO: make client_id an env variable and make the redirect url a env variable already in app state
    //TODO: Add correct scopes
    let redirect_url = format!("https://id.twitch.tv/oauth2/authorize?response_type=code&client_id=9wexo28wfkm476ztq18vafo6xio5la&redirect_uri=https://localhost:8080/auth&scope=moderator:manage:chat_messages&state={}", state);

    HttpResponse::MovedPermanently().insert_header(("Location",redirect_url)).cookie(cookie).finish()

}


#[get("/")]
async fn index(_data: web::Data<AppState>) -> impl Responder {
    // HttpResponse::MovedPermanently().insert_header(("Location","https://id.twitch.tv/oauth2/authorize?response_type=code&client_id=9wexo28wfkm476ztq18vafo6xio5la&redirect_uri=http://localhost:8080/auth&scope=moderator:manage:chat_messages")).finish()
    // data.tx.send(Message::Text("PRIVMSG #chloe_dev_rust :Hello from Actix!".into())).await.unwrap();
    // data.blacklist.words.lock().unwrap().insert("test".to_string());
    format!("Hello from Actix!")
}





fn generate_state() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect::<String>()    
}
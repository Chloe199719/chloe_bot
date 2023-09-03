#![allow(unused_imports)]
#![allow(dead_code)]

use std::sync::Arc;
use actix_web::{get, App, HttpResponse, HttpServer, Responder, web, http::header};
use openssl::ssl::{SslAcceptor, SslMethod};
use serde::Deserialize;
use tokio_tungstenite::tungstenite::Message;


// Local imports
use crate::websocket::moderation::Blacklist;
struct AppState {
    tx : async_channel::Sender<Message>,
    blacklist: Arc<Blacklist>
}


pub async fn start_server( tx: async_channel::Sender<Message>, blacklist: Arc<Blacklist>) {
    let mut builder =SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file("key.pem", openssl::ssl::SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    let app_state = web::Data::new(AppState {
        tx,
        blacklist
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
    HttpResponse::MovedPermanently().insert_header(("Location","https://id.twitch.tv/oauth2/authorize?response_type=code&client_id=9wexo28wfkm476ztq18vafo6xio5la&redirect_uri=http://localhost:8080/auth&scope=moderator:manage:chat_messages")).finish()

}


#[get("/")]
async fn index(_data: web::Data<AppState>) -> impl Responder {
    // HttpResponse::MovedPermanently().insert_header(("Location","https://id.twitch.tv/oauth2/authorize?response_type=code&client_id=9wexo28wfkm476ztq18vafo6xio5la&redirect_uri=http://localhost:8080/auth&scope=moderator:manage:chat_messages")).finish()
    // data.tx.send(Message::Text("PRIVMSG #chloe_dev_rust :Hello from Actix!".into())).await.unwrap();
    // data.blacklist.words.lock().unwrap().insert("test".to_string());
    format!("Hello from Actix!")
}


#[derive(Deserialize,Debug)]
struct QueryAuth {
    state: String,
    code: String,
    scope: String,
}

#[get("/auth")]
async fn auth_token(_data: web::Data<AppState>, info:web::Query<QueryAuth>) -> impl Responder {
  
    println!("{:#?}", info);
    format!("Hello from Actix!")
}
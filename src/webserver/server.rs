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
struct AppState {
    tx : async_channel::Sender<Message>,
    blacklist: Arc<Blacklist>,
    req_client: reqwest::Client,
    client_id: String,
    client_secret: String,
    pg_pool: Pool<Postgres>,
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
    // TODO: make client_id an env variable and make the redirect url a env variable
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


#[derive(Deserialize,Debug)]
struct QueryAuth {
    state: String,
    code: String,
    scope: String,
}

#[get("/auth")]
async fn auth_token(req: HttpRequest ,info:web::Query<QueryAuth>, data: web::Data<AppState>) -> impl Responder {
    if let Some(cookie) = req.cookie("oauth_state") {
        if cookie.value() != info.state {
            return HttpResponse::BadRequest().body("Invalid State");
        }
    } else {
        return HttpResponse::BadRequest().body("Invalid State");
    }

    //TODO: uri from state 
    let params = [
        ("client_id", data.client_id.as_str()),
        ("client_secret", data.client_secret.as_str()),
        ("code", info.code.as_str()),
        ("grant_type", "authorization_code"),
        ("redirect_uri", "https://localhost:8080/auth"),
    ];
   
    // TODO : Handle errors
    let token_res:AuthResponse = data.req_client.post("https://id.twitch.tv/oauth2/token").form(&params).send().await.unwrap().json().await.unwrap();
    let validate_res:VerifyResponse = data.req_client.get("https://id.twitch.tv/oauth2/validate").bearer_auth(&token_res.access_token).send().await.unwrap().json().await.unwrap();
    let scopes:String = validate_res.scopes.join(",");
    // TODO: Store tokens in a database handle error


    let record:Result<ReturnSQL, sqlx::Error> =sqlx::query_as(
        r#"
        SELECT * FROM upsert_user($1,$2,$3,$4,$5,$6)
        "#

    )
    .bind(&validate_res.login)
    .bind(&validate_res.user_id)
    .bind(&scopes)
    .bind(&token_res.access_token)
    .bind(&token_res.refresh_token)
    .bind(&token_res.expires_in)
    .fetch_one(&data.pg_pool).await;
     match record {
         Ok(row) => {
            // Handle 
            match row.success {
                Some(true) => {
                    let mut  map = HashMap::new();
                    let oldtoken = row.old_access_token.unwrap().clone();
                    map.insert("client_id", data.client_id.as_str());
                    map.insert("token", &oldtoken.as_str());
                    let res=    data.req_client.post("https://id.twitch.tv/oauth2/revoke").form(&map).send().await.unwrap();
                    println!("{:#?}", res);
                },
                Some(false) => {
                    println!("User Inserted");
                },
                None => {
                   
                }
            }
          },
         Err(sqlx::Error::RowNotFound) =>{
            println!("Row not found");
         },
        Err(e) => {
            tracing::error!("Error inserting user into database: {:?}", e);
        }
         
     }

   
    //Handle Login 
    //TODO: start listening to chat messages for this user
    //TODO: Set bot to mod for this user
    // println!("{:#?}, {:#?}", token_res,validate_res);

    HttpResponse::Ok().body("Hello")
}

// #[derive(Deserialize,Debug)]
#[derive(sqlx::FromRow,Debug)]
struct ReturnSQL{
    success: Option<bool>,
    old_name: Option<String>,
    old_scopes: Option<String>,
    old_refresh_token: Option<String>,
    old_access_token: Option<String>,
    old_expires_in: Option<i32>,
    old_created_at: Option<chrono::NaiveDateTime>,
    old_updated_at: Option<chrono::NaiveDateTime>,
}



#[derive(Deserialize,Debug)]
struct AuthResponse {
    access_token: String,
    refresh_token: String,
    expires_in: i32,
    scope: Vec<String>,
    token_type: String,
}
#[derive(Deserialize,Debug)]
pub struct VerifyResponse {
    client_id: String,
    login: String,
    scopes: Vec<String>,
    user_id: String,
    expires_in: u64,
}




fn generate_state() -> String {
    let random_state: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    // Store this state value in a session or a secure cookie
    random_state
}
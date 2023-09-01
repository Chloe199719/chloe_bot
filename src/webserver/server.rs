use std::sync::Arc;
use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};
use tokio_tungstenite::tungstenite::Message;


// Local imports
use crate::websocket::moderation::Blacklist;
struct AppState {
    tx : async_channel::Sender<Message>,
    blacklist: Arc<Blacklist>
}


pub async fn start_server( tx: async_channel::Sender<Message>, blacklist: Arc<Blacklist>) {
    let x  =String::from("test");
    let _y:Arc<str> = Arc::from(x);
    let app_state = web::Data::new(AppState {
        tx,
        blacklist
    });
    HttpServer::new(move|| {
        App::new().app_data(app_state.clone()).service(index)
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
    .unwrap();

}


#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    data.tx.send(Message::Text("PRIVMSG #chloe_dev_rust :Hello from Actix!".into())).await.unwrap();
    data.blacklist.words.lock().unwrap().insert("test".to_string());
    format!("Hello from Actix!")
}
use std::sync::Arc;
use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};
use tungstenite::Message;

// Local imports
use crate::websocket::moderation::Blacklist;
struct AppState {
    tx : futures_channel::mpsc::UnboundedSender<Message>,
    blacklist: Arc<Blacklist>
}


pub async fn start_server( tx: futures_channel::mpsc::UnboundedSender<Message>, blacklist: Arc<Blacklist>) {


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
    data.tx.unbounded_send(Message::Text("PRIVMSG #chloe_dev_rust :Hello from Actix!".into())).unwrap();
    data.blacklist.words.lock().unwrap().insert("test".to_string());
    format!("Hello from Actix!")
}
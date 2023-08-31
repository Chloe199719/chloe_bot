#![allow(dead_code, unused_imports)]
use std::collections::HashSet;
use std::process::exit;
use std::sync::{Mutex, Arc};
use std::thread;

use chloe_bot::websocket::message_parser::TwitchMessage;
use chloe_bot::websocket::moderation::{message_processing, Blacklist};
use futures_util::{ future, pin_mut, StreamExt };
use tokio::io::{ AsyncReadExt, AsyncWriteExt };
use tokio::select;
use tokio::signal::unix::{SignalKind, signal,
Signal};
use tokio_tungstenite::{ connect_async, tungstenite::protocol::Message };
use dotenv::dotenv;
use tokio::runtime::Runtime;
use actix_web::rt::{System, self};
#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let auth_token = std::env::var("AUTH_TOKEN").expect("AUTH_TOKEN not set");
    let parse_token = format!("PASS oauth:{}", auth_token);
    let url = url::Url::parse("ws://irc-ws.chat.twitch.tv:80").unwrap();
    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();

    // Moderation Thread
    let black_list = Arc::new(Blacklist::new(vec!["kekw", "pog","eskay"]));
    let (comssender, coms) = futures_channel::mpsc::unbounded();
    tokio::spawn(message_processing(coms, black_list.clone()));
    
    // Stdin Thread
    tokio::spawn(read_stdin(stdin_tx.clone()));
    
    let clone = stdin_tx.clone();
    // Actix Thread
    let actix_thread = thread::spawn(move|| {
        actix_rt::System::new().block_on(start_server(clone, black_list.clone()));
    });
    let mut stream = signal(SignalKind::interrupt()).unwrap();
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");
    let (write, read) = ws_stream.split();

    let stdin_to_ws = stdin_rx.map(Ok).forward(write);
    let ws_to_stdout = {
        read.for_each(|message| async {
            match message {
                Ok(Message::Text(data)) => {
                    if data.starts_with("PING") {
                        stdin_tx
                            .unbounded_send(Message::Text("PONG :tmi.twitch.tv".into()))
                            .unwrap();
                    }
                    println!("{:#?}", data);
                    let message = TwitchMessage::parse_message(data.clone());
                    // if message.command.message.starts_with("!ping") {
                    //     stdin_tx
                    //         .unbounded_send(Message::Text("PRIVMSG #chloe_dev_rust :PONG".into()))
                    //         .unwrap();
                    // }
                    // message_processing(&message);
                    comssender.unbounded_send(message).unwrap();
                    // println!("{:#?}", message);
                }
                Ok(data) => { println!("Received: {:?}", data) }
                Err(e) => eprintln!("Error: {:?}", e),
            }

            // tokio::io::stdout().write_all(&data).await.unwrap();
        })          
    };
   
    stdin_tx
        .unbounded_send(
            Message::Text("CAP REQ :twitch.tv/membership twitch.tv/tags twitch.tv/commands".into())
        )
        .unwrap();
    stdin_tx.unbounded_send(Message::Text(parse_token)).unwrap();
    stdin_tx.unbounded_send(Message::Text(String::from("NICK chloe_dev_rust"))).unwrap();
    stdin_tx.unbounded_send(Message::Text("JOIN #eskay, #chloe_dev_rust".into())).unwrap();
    
    
    let ws_task = async {
        pin_mut!(stdin_to_ws, ws_to_stdout);
        future::select(stdin_to_ws, ws_to_stdout).await;
    };

    // Wait for the WebSocket tasks to finish or Ctrl+C, whichever comes first
    let ctrl_c_task = stream.recv();

    tokio::select! {
        _ = ws_task => {
            eprintln!("WebSocket tasks completed.");
        }
        _ = ctrl_c_task => {
            eprintln!("Ctrl+C received.");
        }
    }

    // Now, you can close the Actix server and any other tasks if necessary
    actix_thread.join().unwrap();
    
    

    exit(0);
}
struct AppState {
    tx : futures_channel::mpsc::UnboundedSender<Message>,
    blacklist: Arc<Blacklist>
}

use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};
async fn start_server( tx: futures_channel::mpsc::UnboundedSender<Message>, blacklist: Arc<Blacklist>) {
    use actix_web::{get, App, HttpServer, Responder};

    #[get("/")]
    async fn index(data: web::Data<AppState>) -> impl Responder {
        data.tx.unbounded_send(Message::Text("PRIVMSG #chloe_dev_rust :Hello from Actix!".into())).unwrap();
        data.blacklist.words.lock().unwrap().insert("test".to_string());
        format!("Hello from Actix!")
    }
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
// Our helper method which will read data from stdin and send it along the
// sender provided.
async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) {
    let mut stdin = tokio::io::stdin();
    stdin.read(&mut [0]).await.unwrap();
    loop {
        let mut buf = vec![0; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => {
                break;
            }
            Ok(n) => n,
        };
        buf.truncate(n);
        let s = String::from_utf8(buf).unwrap();
        // let s = format!("PRIVMSG #chloe_dev_rust :{}", String::from_utf8(buf).unwrap());
        println!("Sending: {}", s);
        tx.unbounded_send(Message::Text(s.to_string())).unwrap();
    }
}

#![allow(dead_code)]
// #![allow(unused_imports)]

use std::process::exit;
use std::sync::Arc;
use std::thread;


use tokio::io::AsyncReadExt;

use tokio::signal::unix::{SignalKind, signal};
use tokio_tungstenite::tungstenite::protocol::Message;
use dotenv::dotenv;

// Local imports
use chloe_bot::websocket::client::web_socket_client;
use chloe_bot::webserver::server::start_server;
use chloe_bot::websocket::moderation::{message_processing, Blacklist};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let mut stream = signal(SignalKind::interrupt()).unwrap();
    
    let (stdin_tx, stdin_rx) = async_channel::unbounded();

    // Moderation Thread
    let black_list = Arc::new(Blacklist::new(vec!["kekw", "pog","eskay"]));
    let (moderator_sender, moderator_receiver) = futures_channel::mpsc::unbounded();
    tokio::spawn(message_processing(moderator_receiver, black_list.clone()));
    

    // Stdin Thread
    tokio::spawn(read_stdin(stdin_tx.clone()));
    
    let clone = stdin_tx.clone();
    // Actix Thread
    let actix_thread = thread::spawn(move|| {
        actix_rt::System::new().block_on(start_server(clone, black_list.clone()));
    });

    tokio::spawn(web_socket_client((stdin_tx.clone(), stdin_rx.clone()),moderator_sender.clone()));
    // let socket =  web_socket_client((stdin_tx.clone(), stdin_rx),moderator_sender.clone());



    // Wait for the WebSocket tasks to finish or Ctrl+C, whichever comes first
    let ctrl_c_task = stream.recv();

    tokio::select! {
        // _ = ws_task => {
        //     eprintln!("WebSocket tasks completed.");
        // }
        _ = ctrl_c_task => {
            eprintln!("Ctrl+C received.");
        }
        // _ = socket => {
        //     eprintln!("Socket task completed.");
        // }
    }

    //Close A
    actix_thread.join().unwrap();
    
    

    exit(0);
}

// TODO: this is a hacky way to read stdin, but it works for now. Should not be needed in production.
async fn read_stdin(tx: async_channel::Sender<Message>) {
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
        tx.send(Message::Text(s.to_string())).await.unwrap();
    }
}

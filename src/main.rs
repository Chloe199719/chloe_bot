#![allow(dead_code)]
// #![allow(unused_imports)]

use std::process::exit;
use std::sync::Arc;
use std::{thread, env};




use sqlx::PgPool;
use tokio::io::AsyncReadExt;

use tokio::signal::unix::{SignalKind, signal};
use tokio_tungstenite::tungstenite::protocol::Message;
use dotenv::dotenv;

// Local imports
use chloe_bot::telemetry::telemetry::{get_subscriber, init_subscriber};
use chloe_bot::websocket::client::web_socket_client;
use chloe_bot::webserver::server::start_server;
use chloe_bot::websocket::moderation::{message_processing, Blacklist};
use tracing::info;

#[tracing::instrument]
#[tokio::main]
async fn main() {
    dotenv().ok();
    //
   
    
    // let time = chrono::Local::now().to_string();
    // let file = std::fs::File::create(format!("logs/chloe-bot-{}.log",time)).expect("Unable to create log file");
    let subscriber = get_subscriber("Chloe Bot".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let mut stream = signal(SignalKind::interrupt()).unwrap();
    
    // Database Connections
    let connection_pool = PgPool::connect(
        &env::var("DATABASE_URL").expect("DATABASE_URL not set"),
    ).await.expect("Failed to connect to Postgres.");


    // Channels
    info!("Creating Channels");
    let (stdin_tx, stdin_rx) = async_channel::unbounded();
    let (moderator_sender, moderator_receiver) = futures_channel::mpsc::unbounded();

    // Moderation Thread
    info!("Creating Moderation Thread");
    let black_list = Arc::new(Blacklist::new(vec!["kekw", "pog","eskay"]));
    let moderator_thread = tokio::spawn(message_processing(moderator_receiver, black_list.clone()));
    

    // Stdin Thread
    let std_in_thread = tokio::spawn(read_stdin(stdin_tx.clone()));
    
    let clone = stdin_tx.clone();
    let connection_pool_web = connection_pool.clone();
    // Actix Thread
    let actix_thread = thread::spawn(move|| {
        actix_rt::System::new().block_on(start_server(clone, black_list.clone(),connection_pool_web));
    });
    // WebSocket Thread
    let connection_pool_socket = connection_pool.clone();
    let web_socket_thread = tokio::spawn(web_socket_client((stdin_tx.clone(), stdin_rx.clone()),moderator_sender.clone(), connection_pool_socket));




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

    //Close Threads
    actix_thread.join().unwrap();
    std_in_thread.abort();
    web_socket_thread.abort();
    moderator_thread.abort();
    

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

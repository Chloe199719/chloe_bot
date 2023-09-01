use futures_util::{StreamExt, pin_mut, future};


use tokio_tungstenite::{ connect_async, tungstenite::protocol::Message };

use crate::websocket::message_parser::TwitchMessage;

pub async fn web_socket_client((stdin_tx, stdin_rx): (futures_channel::mpsc::UnboundedSender<Message>, futures_channel::mpsc::UnboundedReceiver<Message>), moderation_sender : futures_channel::mpsc::UnboundedSender<TwitchMessage>) -> () {
    let auth_token = std::env::var("AUTH_TOKEN").expect("AUTH_TOKEN not set");
    let parse_token = format!("PASS oauth:{}", auth_token);

    // TODO: Make ENV Maybe? or switch  using tls
    let url = url::Url::parse("ws://irc-ws.chat.twitch.tv:80").unwrap();

    //TODO: Handle this error more gracefully
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    let (write, read) = ws_stream.split();
    let stdin_to_ws = stdin_rx.map(Ok).forward(write);
 
    let ws_to_stdout = {    
        read.for_each(|message| async{
            match message {
                Ok(Message::Text(data)) => {
                    if data.starts_with("PING") {
                        stdin_tx
                            .unbounded_send(Message::Text("PONG :tmi.twitch.tv".into()))
                            .unwrap();
                    }
                    println!("{:#?}", data);
                    let message = TwitchMessage::parse_message(data.clone());

                    moderation_sender.unbounded_send(message).unwrap();
                    // println!("{:#?}", message);
                }
                Ok(data) => { println!("Received: {:?}", data) }
                Err(e) => eprintln!("Error: {:?}", e),
            }

  
        })          
    };
    // MOVE THIS SOMEWHERE ELSE
    stdin_tx
    .unbounded_send(
        Message::Text("CAP REQ :twitch.tv/membership twitch.tv/tags twitch.tv/commands".into())
    )
    .unwrap();
    stdin_tx.unbounded_send(Message::Text(parse_token)).unwrap();
    stdin_tx.unbounded_send(Message::Text(String::from("NICK chloe_dev_rust"))).unwrap();
    stdin_tx.unbounded_send(Message::Text("JOIN #theprimeagen, #chloe_dev_rust".into())).unwrap();
    let ws_task = async {
        pin_mut!(stdin_to_ws, ws_to_stdout);
        future::select(stdin_to_ws, ws_to_stdout).await;
    };
    return ws_task.await;
}
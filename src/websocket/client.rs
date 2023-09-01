use futures_util::{StreamExt, pin_mut, future};
use tokio_tungstenite::{ connect_async, tungstenite::protocol::Message };

use crate::websocket::message_parser::TwitchMessage;

    


pub async fn web_socket_client((stdin_tx, stdin_rx): (async_channel::Sender<Message>, async_channel::Receiver<Message>), moderation_sender : futures_channel::mpsc::UnboundedSender<TwitchMessage>) -> () {
    let auth_token = std::env::var("AUTH_TOKEN").expect("AUTH_TOKEN not set");
    let parse_token = format!("PASS oauth:{}", auth_token);

    // TODO: Make ENV Maybe? or switch  using tls
    let url = url::Url::parse("ws://irc-ws.chat.twitch.tv:80").unwrap();

    let mut backoff:u64 = 1;
    let max_backoff:u64 = 300;
    let factor:u64 = 2;


    //TODO: Handle this error more gracefully
    loop {
        
        match connect_async(url.clone()).await {
            Ok((ws_stream,_))=>{
                backoff = 1;
                let (write,mut read) = ws_stream.split();
                let stdin_to_ws = stdin_rx.clone().map(Ok).forward(write);
                let ws_to_stdout=  async {while let Some(message) = read.next().await {
                    match message {
                        Ok(Message::Text(data)) => {
                            if data.starts_with("PING") {
                            
                                stdin_tx
                                    .send(Message::Text("PONG :tmi.twitch.tv".into())).await.unwrap();
                                    
                            }
                            println!("{:#?}", data);
                            let message = TwitchMessage::parse_message(data.clone());

                            moderation_sender.unbounded_send(message).unwrap();
                            // println!("{:#?}", message);
                        }
                        Ok(Message::Close(_)) => {
                            println!("Received Close from Server");
                            break;
                        }
                        Ok(data) => { println!("Received: {:?}", data) }
                        Err(e) => {
                            eprintln!("Error: {:?}", e);
                            break;
                        },
                    }
                    
                }};

                stdin_tx
                .send(
                    Message::Text("CAP REQ :twitch.tv/membership twitch.tv/tags twitch.tv/commands".into())
                )
                .await.unwrap();
            
                stdin_tx.send(Message::Text(parse_token.clone())).await.unwrap();
                stdin_tx.send(Message::Text(String::from("NICK chloe_dev_rust"))).await.unwrap();
                stdin_tx.send(Message::Text("JOIN #theprimeagen, #chloe_dev_rust".into())).await.unwrap();

                let ws_task = async{
                    pin_mut!(stdin_to_ws, ws_to_stdout);
                    future::select(stdin_to_ws, ws_to_stdout).await;
                };
                ws_task.await;
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
                println!("Reconnecting in {} seconds", backoff);
            }
        } 
        println!("Reconnecting in {} seconds", backoff);
        tokio::time::sleep(tokio::time::Duration::from_secs(backoff)).await;
        backoff = (backoff * factor).min(max_backoff);
      

        
      
    }
}
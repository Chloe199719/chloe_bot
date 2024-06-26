use futures_util::{future, pin_mut, StreamExt};
use sqlx::PgPool;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

use crate::websocket::{
    bot_commands_parser::parse_command, init::web_socket_init, message_parser::TwitchMessage,
};

pub async fn web_socket_client(
    (stdin_tx, stdin_rx): (
        async_channel::Sender<Message>,
        async_channel::Receiver<Message>,
    ),
    moderation_sender: futures_channel::mpsc::UnboundedSender<TwitchMessage>,
    pool: PgPool,
) -> () {
    let auth_token = std::env::var("AUTH_TOKEN").expect("AUTH_TOKEN not set");
    let parse_token = format!("PASS oauth:{}", auth_token);

    // TODO: Make ENV Maybe? or switch  using tls
    let url = url::Url::parse("wss://irc-ws.chat.twitch.tv:443").unwrap();

    let mut backoff: u64 = 1;
    let max_backoff: u64 = 300;
    let factor: u64 = 2;

    loop {
        match connect_async(url.clone()).await {
            Ok((ws_stream, _)) => {
                backoff = 1;
                let (write, mut read) = ws_stream.split();
                let stdin_to_ws = stdin_rx.clone().map(Ok).forward(write);
                let ws_to_stdout = async {
                    while let Some(message) = read.next().await {
                        match message {
                            Ok(Message::Text(data)) => {
                                if data.starts_with("PING") {
                                    stdin_tx
                                        .send(Message::Text("PONG :tmi.twitch.tv".into()))
                                        .await
                                        .unwrap();
                                }
                                // println!("{:#?}", data);
                                let message = TwitchMessage::parse_message(data.clone());
                                if message.command.command
                                    == crate::websocket::message_parser::MessageTypes::PRIVMSG
                                {
                                    let (text, channel, sender) = (
                                        &message.command.message,
                                        &message.command.channel,
                                        &message.source.clone().unwrap().nick,
                                    );
                                    tracing::info!(
                                        "Message: {:?} in  Channel: {:?}, From: {}",
                                        text,
                                        channel,
                                        sender
                                    );
                                    if text.starts_with("!") {
                                        parse_command(&text, stdin_tx.clone(), &channel).await;
                                    }

                                    moderation_sender.unbounded_send(message.clone()).unwrap();
                                }

                                // println!("{:#?}", message);
                            }
                            Ok(Message::Close(_)) => {
                                println!("Received Close from Server");
                                break;
                            }
                            Ok(data) => {
                                println!("Received: {:?}", data);
                            }
                            Err(e) => {
                                eprintln!("Error: {:?}", e);
                                break;
                            }
                        }
                    }
                };

                web_socket_init(stdin_tx.clone(), &parse_token, &pool).await;
                // stdin_tx.send(Message::Text("JOIN #naowh, #chloe_dev_rust".into())).await.unwrap();

                let ws_task = async {
                    pin_mut!(stdin_to_ws, ws_to_stdout);
                    future::select(stdin_to_ws, ws_to_stdout).await;
                };
                ws_task.await;
            }
            Err(e) => {
                tracing::error!("Error: {:?}", e);
            }
        }
        tracing::info!("Reconnecting in {} seconds", backoff);
        tokio::time::sleep(tokio::time::Duration::from_secs(backoff)).await;
        backoff = (backoff * factor).min(max_backoff);
    }
}

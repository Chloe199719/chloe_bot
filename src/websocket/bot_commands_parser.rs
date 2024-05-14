use tokio_tungstenite::tungstenite::Message;

pub async fn parse_command(
    message: &str,
    message_sender: async_channel::Sender<Message>,
    channel: &str,
) {
    match message {
        "!ping" => {
            message_sender
                .send(Message::Text(format!("PRIVMSG {} :PONG", channel)))
                .await
                .unwrap();
        }
        _ => {}
    }
}

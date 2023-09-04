use sqlx::{  Postgres, Pool };

use tokio_tungstenite::tungstenite::Message;

pub async fn web_socket_init(stdin_tx: async_channel::Sender<Message>, parse_token: String) {
    stdin_tx
        .send(
            Message::Text("CAP REQ :twitch.tv/membership twitch.tv/tags twitch.tv/commands".into())
        ).await
        .unwrap();

    stdin_tx.send(Message::Text(parse_token.clone())).await.unwrap();
    stdin_tx.send(Message::Text(String::from("NICK chloe_dev_rust"))).await.unwrap();
    stdin_tx.send(Message::Text("JOIN #naowh, #chloe_dev_rust".into())).await.unwrap();
}

pub async fn join_channel(stdin_tx: async_channel::Sender<Message>, channel: String) {
    stdin_tx.send(Message::Text(format!("JOIN #{}", channel))).await.unwrap();
}

pub async fn leave_channel(stdin_tx: async_channel::Sender<Message>, channel: String) {
    stdin_tx.send(Message::Text(format!("PART #{}", channel))).await.unwrap();
}

pub async fn get_channels_to_join(pool: Pool<Postgres>) -> Vec<String> {
    let data = sqlx
        ::query!(r#"
    SELECT name FROM users
   "#)
        .fetch_all(&pool).await
        .unwrap().into_iter().map(|row| {
            row.name
        }).collect::<Vec<String>>();
    data
}
// to be run in different thread
pub async fn loop_through_channels_to_join(
    stdin_tx: async_channel::Sender<Message>,
    channels: Vec<String>
) {
    let mut cooldown:u8 = 0;
    for channel in channels {
        join_channel(stdin_tx.clone(), channel).await;
        cooldown += 1;
        if cooldown == 10 {
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            cooldown = 0;
        }
    }
    tracing::info!("Joined all channels");
}

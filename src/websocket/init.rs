use sqlx::{  Postgres, Pool };

use tokio_tungstenite::tungstenite::Message;
#[tracing::instrument(skip(stdin_tx, parse_token, pool))]
pub async fn web_socket_init(stdin_tx: async_channel::Sender<Message>, parse_token: &String, pool: &Pool<Postgres>) {
    stdin_tx
        .send(
            Message::Text("CAP REQ :twitch.tv/membership twitch.tv/tags twitch.tv/commands".into())
        ).await
        .unwrap();

    stdin_tx.send(Message::Text(parse_token.clone())).await.unwrap();
    stdin_tx.send(Message::Text(String::from("NICK chloe_dev_rust"))).await.unwrap();
    //FIXME: Remove mut keyword and manual channel joins
    let mut channel_list = get_channels_to_join(&pool).await;
    channel_list.push(String::from("maximum"));
    tokio::spawn(loop_through_channels_to_join(stdin_tx.clone(), channel_list));
    tracing::info!("Finished init");
}

pub async fn join_channel(stdin_tx: async_channel::Sender<Message>, channel: &String) {
    stdin_tx.send(Message::Text(format!("JOIN #{}", channel))).await.unwrap();
}

pub async fn leave_channel(stdin_tx: async_channel::Sender<Message>, channel: String) {
    stdin_tx.send(Message::Text(format!("PART #{}", channel))).await.unwrap();
}

//TODO: Error handling 
//Probably create a backoff system when reaches cap  panic and crash
#[tracing::instrument(skip(pool))]
pub async fn get_channels_to_join(pool: &Pool<Postgres>) -> Vec<String> {
    let data = sqlx
        ::query!(r#"
    SELECT name FROM users
   "#)
        .fetch_all(pool).await
        .unwrap().into_iter().map(|row| {
            row.name
        }).collect::<Vec<String>>();
    tracing::info!("Channels to join: {:?}", data);    
    data
}
// to be run in different thread
#[tracing::instrument(skip(stdin_tx, channels))]
pub async fn loop_through_channels_to_join(
    stdin_tx: async_channel::Sender<Message>,
    channels: Vec<String>
) {
    let mut cooldown:u8 = 0;
    for channel in channels {
        join_channel(stdin_tx.clone(), &channel).await;
        tracing::info!("Joined channel: {}", channel);
        cooldown += 1;
        if cooldown == 10 {
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            cooldown = 0;
        }
    }
    tracing::info!("Joined all channels");
}

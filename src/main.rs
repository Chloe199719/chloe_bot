#![allow(dead_code, unused_imports)]
use std::process::exit;
use std::thread;

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
    tokio::spawn(read_stdin(stdin_tx.clone()));
    
    let clone = stdin_tx.clone();
    let actix_thread = thread::spawn(|| {
        actix_rt::System::new().block_on(start_server(clone));
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
                    if message.command.message.starts_with("!ping") {
                        stdin_tx
                            .unbounded_send(Message::Text("PRIVMSG #chloe_dev_rust :PONG".into()))
                            .unwrap();
                    }
                    println!("{:#?}", message);
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
    stdin_tx.unbounded_send(Message::Text("JOIN #chloe_dev_rust".into())).unwrap();
    
    
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
    
    
    // If you still want to force exit:
    // exit(0);
}
struct AppState {
    tx : futures_channel::mpsc::UnboundedSender<Message>,
}

use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};
async fn start_server( tx: futures_channel::mpsc::UnboundedSender<Message>) {
    use actix_web::{get, App, HttpServer, Responder};

    #[get("/")]
    async fn index(data: web::Data<AppState>) -> impl Responder {
        data.tx.unbounded_send(Message::Text("PRIVMSG #chloe_dev_rust :Hello from Actix!".into())).unwrap();
        format!("Hello from Actix!")
    }
   
    HttpServer::new(move|| {
        App::new().app_data(web::Data::new(AppState{
            tx: tx.clone(),
        })).service(index)
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
#[derive(Debug, Clone)]
enum MessageTypes {
    PING,
    PRIVMSG,
    ROOMSTATE,
    CLEARCHAT,
    CLEARMSG,
    HOSTTARGET,
    RECONNECT,
    USERNOTICE,
    USERSTATE,
    GLOBALUSERSTATE,
    UNKNOWN,
}

#[derive(Debug, Clone)]
struct TwitchMessage {
    tags: Vec<Tags>,
    command: Commands,
    params: Vec<String>,
    source: Option<Source>,
}
#[derive(Debug, Clone)]
struct Commands {
    command: MessageTypes,
    channel: String,
    message: String,
}
impl TwitchMessage {
    fn parse_message(message: String) -> TwitchMessage {
        let mut tags: Vec<Tags> = Vec::new();
        let params: Vec<String> = Vec::new();
        let mut source: Option<Source> = None;

        let mut split = message.split(" ");
        let first = split.next().unwrap();
        if first.starts_with("@") {
            let tags_split = first.split(";");
            for tag in tags_split {
                let mut tag_split = tag.split("=");
                let key = tag_split.next().unwrap();
                let value = tag_split.next().unwrap();
                match key {
                    "badges" => {
                        let mut badges = Vec::new();
                        let badges_split = value.split(",");
                        for badge in badges_split {
                            let mut badge_split = badge.split("/");
                            let name = badge_split.next().unwrap();
                            let version = 0;
                            let amount = match badge_split.next() {
                                Some(x) => x.parse::<i32>().ok(),

                                None => {
                                    break;
                                }
                            };

                            let badge = match name {
                                "broadcaster" =>
                                    Badge::Broadcaster(BadgeInfo {
                                        name: name.to_string(),
                                        version: version.to_string(),
                                        amount,
                                    }),
                                "moderator" =>
                                    Badge::Moderator(BadgeInfo {
                                        name: name.to_string(),
                                        version: version.to_string(),
                                        amount,
                                    }),
                                "subscriber" =>
                                    Badge::Subscriber(BadgeInfo {
                                        name: name.to_string(),
                                        version: version.to_string(),
                                        amount,
                                    }),
                                "vip" =>
                                    Badge::VIP(BadgeInfo {
                                        name: name.to_string(),
                                        version: version.to_string(),
                                        amount,
                                    }),
                                "bits" =>
                                    Badge::Bits(BadgeInfo {
                                        name: name.to_string(),
                                        version: version.to_string(),
                                        amount,
                                    }),
                                "premium" =>
                                    Badge::Premium(BadgeInfo {
                                        name: name.to_string(),
                                        version: version.to_string(),
                                        amount,
                                    }),
                                "bits-leader" =>
                                    Badge::BitsLeader(BadgeInfo {
                                        name: name.to_string(),
                                        version: version.to_string(),
                                        amount,
                                    }),
                                "staff" =>
                                    Badge::Staff(BadgeInfo {
                                        name: name.to_string(),
                                        version: version.to_string(),
                                        amount,
                                    }),
                                "admin" =>
                                    Badge::Admin(BadgeInfo {
                                        name: name.to_string(),
                                        version: version.to_string(),
                                        amount,
                                    }),
                                "global_mod" =>
                                    Badge::GlobalMod(BadgeInfo {
                                        name: name.to_string(),
                                        version: version.to_string(),
                                        amount,
                                    }),
                                _ =>
                                    Badge::Unknown(BadgeInfo {
                                        name: name.to_string(),
                                        version: version.to_string(),
                                        amount,
                                    }),
                            };
                            badges.push(badge);
                        }
                        tags.push(Tags::Badges(badges));
                    }
                    "emotes" => {
                        let mut emotes = Vec::new();
                        if value.is_empty() {
                            tags.push(Tags::Emotes(emotes));
                            continue;
                        }
                        let emotes_split = value.split("/");
                        for emote in emotes_split {
                            emotes.push(Emote {
                                id: emote.to_string(),
                            });
                        }
                        tags.push(Tags::Emotes(emotes));
                    }
                    "subscriber" => {
                        let subscriber = match value {
                            "1" => true,
                            _ => false,
                        };
                        tags.push(Tags::Subscriber(subscriber));
                    }
                    "turbo" => {
                        let turbo = match value {
                            "1" => true,
                            _ => false,
                        };
                        tags.push(Tags::Turbo(turbo));
                    }
                    "color" => {
                        tags.push(Tags::Color(value.to_string()));
                    }
                    "mod" => {
                        let mod_ = match value {
                            "1" => true,
                            _ => false,
                        };
                        tags.push(Tags::Mod(mod_));
                    }
                    "user-id" => {
                        tags.push(Tags::UserID(value.to_string()));
                    }
                    "user-type" => {
                        tags.push(Tags::UserType(value.to_string()));
                    }
                    "display-name" => {
                        tags.push(Tags::DisplayName(value.to_string()));
                    }
                    "id" => {
                        tags.push(Tags::MessageID(value.to_string()));
                    }
                    "tmi-sent-ts" => {
                        tags.push(Tags::TimeSent(value.to_string()));
                    }
                    _ => {}
                }
            }
        }
        //
        if first.starts_with(":") {
            let mut source_split = first.split("!");
            let nick = source_split.next().unwrap().to_string();
            if nick.contains(":tmi") {
                source = Some(Source { nick: "".to_string(), host: nick });
                let command_split = split.next().unwrap();
                let command = match command_split {
                    "PING" => MessageTypes::PING,
                    "PRIVMSG" => MessageTypes::PRIVMSG,
                    "ROOMSTATE" => MessageTypes::ROOMSTATE,
                    "CLEARCHAT" => MessageTypes::CLEARCHAT,
                    "CLEARMSG" => MessageTypes::CLEARMSG,
                    "HOSTTARGET" => MessageTypes::HOSTTARGET,
                    "RECONNECT" => MessageTypes::RECONNECT,
                    "USERNOTICE" => MessageTypes::USERNOTICE,
                    "USERSTATE" => MessageTypes::USERSTATE,
                    "GLOBALUSERSTATE" => MessageTypes::GLOBALUSERSTATE,
                    _ => MessageTypes::UNKNOWN,
                };
                let channel = split.next().unwrap().to_string();
                let mut message = String::new();
                for x in split {
                    message.push_str(x);
                    message.push_str(" ");
                }
                message = message.trim().to_string();

                let command = Commands {
                    command,
                    channel,
                    message,
                };
                // return TwitchMessage Future
                return TwitchMessage {
                    tags,
                    command,
                    params,
                    source,
                };
            }
            let host = match source_split.next() {
                Some(x) => x.to_string(),
                None => "".to_string(),
            };

            source = Some(Source {
                nick,
                host,
            });

            let command_split = split.next().unwrap();
            let command = match command_split {
                "PING" => MessageTypes::PING,
                "PRIVMSG" => MessageTypes::PRIVMSG,
                "ROOMSTATE" => MessageTypes::ROOMSTATE,
                "CLEARCHAT" => MessageTypes::CLEARCHAT,
                "CLEARMSG" => MessageTypes::CLEARMSG,
                "HOSTTARGET" => MessageTypes::HOSTTARGET,
                "RECONNECT" => MessageTypes::RECONNECT,
                "USERNOTICE" => MessageTypes::USERNOTICE,
                "USERSTATE" => MessageTypes::USERSTATE,
                "GLOBALUSERSTATE" => MessageTypes::GLOBALUSERSTATE,
                _ => MessageTypes::UNKNOWN,
            };
            let channel = split.next().unwrap().to_string();
            let mut message = String::new();
            for x in split {
                message.push_str(x);
                message.push_str(" ");
            }
            message = message.trim().to_string();

            let command = Commands {
                command,
                channel,
                message,
            };
            // return TwitchMessage Future
            return TwitchMessage {
                tags,
                command,
                params,
                source,
            };
        }
        if first.starts_with("PING") {
            let command = MessageTypes::PING;
            let mut message = String::new();
            for x in split {
                message.push_str(x);
                message.push_str(" ");
            }
            message = message.trim().to_string();
            let command = Commands {
                command,
                channel: "".to_string(),
                message: message,
            };
            // return TwitchMessage Future
            return TwitchMessage {
                tags,
                command,
                params,
                source,
            };
        }
        // Fix unwraps
        let command_split = split.next().unwrap();
        if command_split.contains(":tmi") {
            source = Some(Source { nick: "".to_string(), host: command_split.to_string() });
            let command_type: MessageTypes = match split.next().unwrap() {
                "PING" => MessageTypes::PING,
                "PRIVMSG" => MessageTypes::PRIVMSG,
                "ROOMSTATE" => MessageTypes::ROOMSTATE,
                "CLEARCHAT" => MessageTypes::CLEARCHAT,
                "CLEARMSG" => MessageTypes::CLEARMSG,
                "HOSTTARGET" => MessageTypes::HOSTTARGET,
                "RECONNECT" => MessageTypes::RECONNECT,
                "USERNOTICE" => MessageTypes::USERNOTICE,
                "USERSTATE" => MessageTypes::USERSTATE,
                "GLOBALUSERSTATE" => MessageTypes::GLOBALUSERSTATE,
                _ => MessageTypes::UNKNOWN,
            };
            let channel = split.next().unwrap().to_string();
            let mut message = String::new();
            for x in split {
                message.push_str(x);
                message.push_str(" ");
            }
            message = message.trim().to_string();
            let command = Commands {
                command: command_type,
                channel,
                message,
            };
            return TwitchMessage {
                tags,
                command,
                params,
                source,
            };
        }
        let mut source_split = command_split.split("!");
        let mut nick = source_split.next().unwrap().to_string();
        nick = nick.trim_start_matches(":").to_string();
        let host = source_split.next().unwrap().to_string();
        source = Some(Source {
            nick,
            host,
        });
        let command = match split.next().unwrap() {
            "PING" => MessageTypes::PING,
            "PRIVMSG" => MessageTypes::PRIVMSG,
            "ROOMSTATE" => MessageTypes::ROOMSTATE,
            "CLEARCHAT" => MessageTypes::CLEARCHAT,
            "CLEARMSG" => MessageTypes::CLEARMSG,
            "HOSTTARGET" => MessageTypes::HOSTTARGET,
            "RECONNECT" => MessageTypes::RECONNECT,
            "USERNOTICE" => MessageTypes::USERNOTICE,
            "USERSTATE" => MessageTypes::USERSTATE,
            "GLOBALUSERSTATE" => MessageTypes::GLOBALUSERSTATE,
            _ => MessageTypes::UNKNOWN,
        };
        let channel = split.next().unwrap().to_string();
        let mut message = String::new();
        for x in split {
            message.push_str(x);
            message.push_str(" ");
        }
        message = message.trim_start_matches(":").to_string();
        message = message.trim().to_string();

        let command = Commands {
            command,
            channel,
            message,
        };
        return TwitchMessage {
            tags,
            command,
            params,
            source,
        };
    }
}

#[derive(Debug, Clone)]
struct Source {
    nick: String,
    host: String,
}
#[derive(Debug, Clone)]
struct Emote {
    id: String,
}
#[derive(Debug, Clone)]
enum Tags {
    Badges(Vec<Badge>),
    Emotes(Vec<Emote>),
    Subscriber(bool),
    Turbo(bool),
    Color(String),
    Mod(bool),
    UserID(String),
    UserType(String),
    DisplayName(String),
    MessageID(String),
    TimeSent(String),
}
#[derive(Debug, Clone)]
enum Badge {
    Broadcaster(BadgeInfo),
    Moderator(BadgeInfo),
    Subscriber(BadgeInfo),
    VIP(BadgeInfo),
    Bits(BadgeInfo),
    Premium(BadgeInfo),
    BitsLeader(BadgeInfo),
    Staff(BadgeInfo),
    Admin(BadgeInfo),
    GlobalMod(BadgeInfo),
    Unknown(BadgeInfo),
}
#[derive(Debug, Clone)]
struct BadgeInfo {
    name: String,
    version: String,
    amount: Option<i32>,
}

mod tests {
    use super::*;

    #[test]
    fn test_parse_message() {
        let message =
            "@badge-info=subscriber/6;badges=subscriber/6;color=#0000FF;display-name=chloe_dev_rust;emotes=;flags=;id=9f3a3f2a-2b1a-4b0a-8e4a-9e2a2a1a1a1a;mod=0;room-id=123456789;subscriber=1;tmi-sent-ts=1612345678901;turbo=0;user-id=123456789;user-type= :chloe_dev_rust!chloe_dev_rust@chloe_dev_rust.tmi.twitch.tv PRIVMSG #naowh :test";
        let x = TwitchMessage::parse_message(message.to_string());
        println!("{:#?}", x);
    }
    #[test]
    fn test_parse_message2() {
        let message = "PING :tmi.twitch.tv";
        let x = TwitchMessage::parse_message(message.to_string());
        println!("{:#?}", x);
    }
    #[test]
    fn test_parse_message3() {
        let message = ":lovingt3s!lovingt3s@lovingt3s.tmi.twitch.tv PRIVMSG #lovingt3s :!dilly";
        let x = TwitchMessage::parse_message(message.to_string());
        println!("{:#?}", x);
    }
    #[test]
    fn test_parse_message4() {
        let message =
            "@badge-info=;badges=partner/1;client-nonce=5c6b9e4d39b7ddc691a0e709f5acb25e;color=#FFFFFF;display-name=Birthdays;emotes=;first-msg=0;flags=;id=55ef0949-91e0-48de-bca8-100eddeb87c9;mod=0;returning-chatter=0;room-id=53649632;subscriber=0;tmi-sent-ts=1693337489988;turbo=0;user-id=45671435;user-type= :birthdays!birthdays@birthdays.tmi.twitch.tv PRIVMSG #naowh :its like you lost 20kg";
        let x = TwitchMessage::parse_message(message.to_string());
        println!("{:#?}", x);
    }
    #[test]
    fn test_parse_message5() {
        let message = ":tmi.twitch.tv 001 <user> :Welcome, GLHF!";
        let x = TwitchMessage::parse_message(message.to_string());
        println!("{:#?}", x);
    }
    #[test]
    fn test_parse_message6() {
        let message =
            "@badge-info=;badges=;client-nonce=7d7d6334634c78473df2236e9e12348a;color=;display-name=kaine_mead;emotes=;first-msg=1;flags=;id=7bae6c75-057c-4d62-9351-9ac8a5bb0560;mod=0;returning-chatter=0;room-id=53649632;subscriber=0;tmi-sent-ts=1693340785356;turbo=0;user-id=948559936;user-type= :kaine_mead!kaine_mead@kaine_mead.tmi.twitch.tv PRIVMSG #naowh :you always main druid tank?\r\n";
        let x = TwitchMessage::parse_message(message.to_string());
        println!("{:#?}", x);
    }
    #[test]
    fn test_parse_message7() {
        let message =
            "@badge-info=subscriber/31;badges=subscriber/24,premium/1;client-nonce=acbc0600e1d35322ed7c790546bf39ef;color=#FF4500;display-name=yaattess;emote-only=1;emotes=492:0-1;first-msg=0;flags=;id=05a544d6-9afa-467a-90a0-b609cfd65f02;mod=0;returning-chatter=0;room-id=53649632;subscriber=1;tmi-sent-ts=1693341298247;turbo=0;user-id=47321353;user-type= :yaattess!yaattess@yaattess.tmi.twitch.tv PRIVMSG #naowh ::O\r\n";
        let x = TwitchMessage::parse_message(message.to_string());
        println!("{:#?}", x);
    }
}

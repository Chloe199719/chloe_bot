#![allow(dead_code, unused_imports)]

#[derive(Debug, Clone, PartialEq)]
pub enum MessageTypes {
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
pub struct TwitchMessage {
    pub tags: Vec<Tags>,
    pub command: Commands,
    pub params: Vec<String>,
    pub source: Option<Source>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Commands {
    pub command: MessageTypes,
    pub channel: String,
    pub message: String,
}
impl TwitchMessage {
    pub fn parse_message(message: String) -> TwitchMessage {
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
pub struct Source {
    pub nick: String,
    pub host: String,
}
#[derive(Debug, Clone,PartialEq)]
pub struct Emote {
    pub id: String,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Tags {
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
#[derive(Debug, Clone, PartialEq)]
pub enum Badge {
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
#[derive(Debug, Clone,PartialEq)]
pub struct BadgeInfo {
    pub name: String,
    pub version: String,
    pub amount: Option<i32>,
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

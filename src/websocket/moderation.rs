

use std::collections::HashMap;

use super::message_parser::{TwitchMessage, MessageTypes, Tags};

pub fn message_processing(message:&TwitchMessage){
    if message.command.command !=  MessageTypes::PRIVMSG {
        return;
    }
    let mut bad_words:HashMap<String, Vec<String>> = HashMap::new();
    bad_words.insert(String::from("kekw"), vec![String::from("#naowh")]);
    bad_words.insert(String::from("pog"), vec![String::from("#naowh")]);

   match bad_words.get(message.command.message.as_str()) {
        Some(value) => {
            if value.contains(&message.command.channel) {
                for tag in &message.tags {
                    match tag {
                        Tags::DisplayName(display_name) => {
                            println!("{} said a bad word in {}", display_name, message.command.channel);
                        },
                        _ => {}                     
                    }
                }
            }
        },
        None => {
           ()
        }
   }

}

use std::collections::HashSet;
use std::sync::{Mutex, Arc};

use futures_util::{future::ready};
use futures_util::StreamExt ;


// use futures_util::StreamExt;
use tokio::time::Instant;

use super::message_parser::{TwitchMessage, MessageTypes, Tags};

pub async fn message_processing(message: futures_channel::mpsc::UnboundedReceiver<TwitchMessage>, blacklist: Arc<Blacklist>) {
    
    
    let _looper = {
        message.for_each(move |message|  {
            let start =Instant::now();
            if message.command.command !=  MessageTypes::PRIVMSG {
                return        ready(());
            }
            if blacklist.contains_blacklist_word(&message.command.message) {
                println!("Message contains blacklisted word");
                let duration = start.elapsed();
                println!("Time elapsed in expensive_function() is: {:?}", duration);
            }
         
            ready(())
        })
    }.await;

}


pub struct Blacklist {
    pub words: Mutex<HashSet<String>>,
}

impl Blacklist {
    pub fn new(words: Vec<&str>) -> Self {
        let words = words.into_iter().map(|word| word.to_lowercase());
        let words = words.collect::<HashSet<_>>();
        Blacklist {  words: Mutex::new( words) }
    }

     fn contains_blacklist_word(&self, message: &str) -> bool {
        let cleaned_message: String = message.chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect::<String>()
            .to_lowercase();
     
        for word in self.words.lock().unwrap().iter() {
            if self.contains_with_garbage(&cleaned_message, word) {
                return true; // Message contains blacklisted word
            }
        }
        
        false
    }

    fn contains_with_garbage(&self, message: &str, word: &str) -> bool {
        let chars: Vec<_> = message.chars().collect();
        let word_chars: Vec<_> = word.chars().collect();
        if word_chars.len() > chars.len() {
            return false;
        }
        for i in 0..=(chars.len() - word_chars.len()) {
            if chars[i..(i + word_chars.len())] == word_chars[..] {
                return true;
            }
        }
        false
    }
}


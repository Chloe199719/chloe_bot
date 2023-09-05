#![allow(unused_imports)]

use std::collections::HashSet;
use std::sync::{Mutex, Arc};

use futures_util::future::ready;
use futures_util::StreamExt ;



use tokio::time::Instant;

use super::message_parser::{TwitchMessage, MessageTypes, Tags};

#[tracing::instrument(skip(message, blacklist))]
pub async fn message_processing(message: futures_channel::mpsc::UnboundedReceiver<TwitchMessage>, blacklist: Arc<Blacklist>) {
  
    
    let _looper = {
        message.for_each(move |message|  {
       
            if message.command.command !=  MessageTypes::PRIVMSG {
                return        ready(());
            }
            if blacklist.contains_blacklist_word(&message.command.message) {
               tracing::info!("Message Blocked: {:?}", message.command.message);
                
            }
         
            ready(())
        })
    }.await;

}

#[derive(Debug)]
pub struct Blacklist {
    pub words: Mutex<HashSet<String>>,
}

impl Blacklist {
    pub fn new(words: Vec<&str>) -> Self {
        let words = words.into_iter().map(|word| word.to_lowercase());
        let words = words.collect::<HashSet<_>>();
        Blacklist {  words: Mutex::new( words) }
    }
//TODO: Might need to not clean the message just make everything lowercase
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

#[cfg(test)]
mod test{
    use super::Blacklist;

    fn word_list () -> Blacklist {
        Blacklist::new(vec!["test", "test2","trans","gay","queer","lesbian","lgbt","lgbtq","lgbtq+","lgbtqia","lgbtqia+","lgbtq","nigga"])
    }

    #[test]
    fn test_blacklisted_words () {
        let blacklist = word_list();
        assert_eq!(blacklist.contains_blacklist_word("test"), true);
        assert_eq!(blacklist.contains_blacklist_word("test2"), true);
    }
    #[test]
    fn test_blacklisted_words_with_garbage () {
        let blacklist = word_list();
        assert_eq!(blacklist.contains_blacklist_word("test!"), true);
        assert_eq!(blacklist.contains_blacklist_word("test2!"), true);
    }
    #[test]
    fn test_blacklisted_words_with_garbage_and_capitalization () {
        let blacklist = word_list();
        assert_eq!(blacklist.contains_blacklist_word("Test!"), true);
        assert_eq!(blacklist.contains_blacklist_word("Test2!"), true);
    }
    #[test]
    fn test_blacklisted_words_with_garbage_and_capitalization_and_whitespace () {
        let blacklist = word_list();
        assert_eq!(blacklist.contains_blacklist_word("Test! "), true);
        assert_eq!(blacklist.contains_blacklist_word("Test2! "), true);
    }
    #[test]
    fn test_blacklisted_words_with_garbage_and_capitalization_and_whitespace_and_alphanumeric () {
        let blacklist = word_list();
        assert_eq!(blacklist.contains_blacklist_word("Test! 123"), true);
        assert_eq!(blacklist.contains_blacklist_word("Test2! 123"), true);
    }
    #[test]
    fn test_notblacklisted_words () {
        let blacklist = word_list();
        assert_eq!(blacklist.contains_blacklist_word("chloe"), false);
        assert_eq!(blacklist.contains_blacklist_word("eskay"), false);
    }
    #[test]
    fn test_notblacklisted_words_with_garbage () {
        let blacklist = word_list();
        assert_eq!(blacklist.contains_blacklist_word("chloe!"), false);
        assert_eq!(blacklist.contains_blacklist_word("eskay!"), false);
    }
}
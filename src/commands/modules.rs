use std::{collections::HashMap, fmt};

#[derive(Debug,Clone,PartialEq)]
pub struct Channels {
    channels: HashMap<String, ChannelCommands>
}
impl Channels {
    pub fn new() -> Self {
        Self {
            channels: HashMap::new()
        }
    }
    pub fn add_channel(&mut self, channel: ChannelCommands) {
        self.channels.insert(channel.channel_name().clone(), channel);
    }
    pub fn remove_channel(&mut self, channel_name: String) {
        self.channels.remove(&channel_name);
    }
    pub fn get_channel(&self, channel_name: String) -> Option<&ChannelCommands> {
        self.channels.get(&channel_name)
    }
    pub fn get_channel_mut(&mut self, channel_name: String) -> Option<&mut ChannelCommands> {
        self.channels.get_mut(&channel_name)
    }

}

#[derive(Debug,Clone,PartialEq)]
pub struct ChannelCommands {
    channel_name: String,
    commands: Vec<Command>,
    global_cooldown: i64
}


impl ChannelCommands {
    pub fn new(channel_name: String, commands: Vec<Command>, global_cooldown: i64) -> Self {
        Self {
            channel_name,
            commands,
            global_cooldown
        }
    }
    pub fn add_command(&mut self, command: Command) {
        self.commands.push(command);
    }
    pub fn remove_command(&mut self, command_name: String) {
        self.commands.retain(|command| command.command_name != command_name);
    }
    pub fn get_command(&self, command_name: String) -> Option<&Command> {
        self.commands.iter().find(|command| command.command_name == command_name)
    }
    pub fn is_on_global_cooldown(&self) -> bool {
        let current_time = chrono::Utc::now().timestamp();

        self.global_cooldown < current_time
    }
    pub fn channel_name(&self) -> &String {
        &self.channel_name
    }
}
#[derive(Debug,Clone,PartialEq)]

pub struct Command {
    command_name: String,
    cooldown: i64,
    next_available: i64,
    command: Commands,
    command_level: UserLevel,
    command_response: Option<String>
}

impl Command {
    pub fn new(command_name: String, cooldown: i64,level: UserLevel) -> Self {
       let command =  match command_name.to_lowercase().as_str()   {
            "Ping"=> { Commands::PING},
            "8ball"=> { Commands::BALL8},
            "ban"=> { Commands::BAN},
            "unban"=> { Commands::UNBAN},
            "timeout"=> { Commands::TIMEOUT},
            "commands"=> { Commands::COMMANDS},
            "addcommand"=> { Commands::ADDCOMMAND},
            "removecommand"=> { Commands::REMOVECOMMAND},
            "followage"=> { Commands::FOLLOWAGE},
            "uptime"=> { Commands::UPTIME},
            _ => { Commands::CUSTOM}
        };

        Self {
            command_name,
            cooldown,
            next_available: 0,
            command_level: level,
            command,
            command_response: None
        }
    }
    pub fn command_name(&self) -> &String {
        &self.command_name
    }
    pub fn command(&self) -> &Commands {
        &self.command
    }
    pub fn is_on_cooldown(&self) -> bool {
        let current_time = chrono::Utc::now().timestamp();

        self.next_available < current_time
    }
    pub fn set_cooldown(&mut self) {
        let current_time = chrono::Utc::now().timestamp();
        self.next_available = current_time + self.cooldown;
    }
}
#[derive(Debug,Clone,PartialEq)]

pub enum Commands {
    PING,
    BALL8,
    BAN,
    UNBAN,
    TIMEOUT,
    COMMANDS,
    ADDCOMMAND,
    REMOVECOMMAND,
    FOLLOWAGE,
    UPTIME,
    CATEGORY,
    CUSTOM,
}
#[derive(Clone,PartialEq)]

pub enum  UserLevel {
    BROADCASTER,
    SUPERMOD,
    MOD,
    VIP,
    SUB,
    EVERYONE
    
}
impl fmt::Debug for  UserLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserLevel::BROADCASTER => write!(f, "Broadcaster"),
            UserLevel::SUPERMOD => write!(f, "Super Moderator"),
            UserLevel::MOD => write!(f, "Moderator"),
            UserLevel::VIP => write!(f, "VIP"),
            UserLevel::SUB => write!(f, "Subscriber"),
            UserLevel::EVERYONE => write!(f, "Everyone"),
        }
    }
}
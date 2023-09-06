
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use rand::Rng;

    fn create_hashmap_of_commands()-> HashMap<String, HashMap<String,bool>>{
        let mut channels = HashMap::new();
        channels.insert(String::from("chloe_rust_dev"), add_commands_to_hashmap());
        channels.insert(String::from("maximum"), add_commands_to_hashmap());
        channels.insert(String::from("boxbox"), add_commands_to_hashmap());
        channels.insert(String::from("robinsongz"), add_commands_to_hashmap());
        channels.insert(String::from("Emiru"), add_commands_to_hashmap());
        channels.insert(String::from("naowh"), add_commands_to_hashmap());
        channels.insert(String::from("nightblue3"), add_commands_to_hashmap());
        channels.insert(String::from("ratirl"), add_commands_to_hashmap());
        channels.insert(String::from("ls"), add_commands_to_hashmap());
        channels.insert(String::from("riotgames"), add_commands_to_hashmap());

        channels.insert(String::from("itshafu"), add_commands_to_hashmap());

        channels.insert(String::from("chloe_rust_dev12"), add_commands_to_hashmap());
        channels.insert(String::from("chloe_rust_dev13"), add_commands_to_hashmap());
        channels.insert(String::from("chloe_rust_dev14"), add_commands_to_hashmap());
        channels.insert(String::from("chloe_rust_dev15"), add_commands_to_hashmap());
        channels.insert(String::from("chloe_rust_dev16"), add_commands_to_hashmap());
        for _ in 0..1000{
            let random_string = generate_random_string(10);
            channels.insert(random_string, add_commands_to_hashmap());
        }


        channels
        
    }
    fn generate_random_string(length: usize) -> String {
        const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let mut rng = rand::thread_rng();
    
        let random_string: String = (0..length)
            .map(|_| {
                let index = rng.gen_range(0..CHARSET.len());
                CHARSET[index] as char
            })
            .collect();
    
        random_string
    }
    fn add_commands_to_hashmap() -> HashMap<String,bool> {

        let mut commands = HashMap::new();
        commands.insert(String::from("test"), true);
        commands.insert(String::from("8ball"), true);
        commands.insert(String::from("accountage"), true);
        commands.insert(String::from("addcom"), true);
        commands.insert(String::from("addquote"), true);
        commands.insert(String::from("addwin"), true);
        commands.insert(String::from("alias"), true);
        commands.insert(String::from("balance"), true);
        commands.insert(String::from("ban"), true);
        commands.insert(String::from("bans"), true);
        commands.insert(String::from("blacklist"), true);
        commands.insert(String::from("blacklistword"), true);
        commands.insert(String::from("blacklistwords"), true);
        commands.insert(String::from("block"), true);
        commands.insert(String::from("blocklist"), true);
        commands.insert(String::from("blocklistword"), true);
        commands.insert(String::from("blocklistwords"), true);
        commands.insert(String::from("blockwords"), true);
        commands.insert(String::from("bonus"), true);
        commands.insert(String::from("bonuspoints"), true);
        commands.insert(String::from("bonuspointsadd"), true);
        commands.insert(String::from("bonuspointsremove"), true);
        commands.insert(String::from("bonusremove"), true);
        commands.insert(String::from("bonuswin"), true);
        commands.insert(String::from("bonuswins"), true);
        commands.insert(String::from("bonuswinsadd"), true);
        commands.insert(String::from("bonuswinsremove"), true);
        commands.insert(String::from("botage"), true);
        commands.insert(String::from("botjoin"), true);
        commands.insert(String::from("botpart"), true);
        commands.insert(String::from("botuptime"), true);
        commands.insert(String::from("chatters"), true);
        commands.insert(String::from("chatterscount"), true);
        commands.insert(String::from("chatterslist"), true);
        commands.insert(String::from("chatterslistadd"), true);
        commands.insert(String::from("chatterslistremove"), true);
        commands.insert(String::from("chatterslisttoggle"), true);
        commands.insert(String::from("chatterslisttoggleadd"), true);
        commands.insert(String::from("chatterslisttoggleremove"), true);
        commands.insert(String::from("chatterslisttoggletoggle"), true);
        commands.insert(String::from("chatterslistview"), true);
        commands.insert(String::from("chattersview"), true);
        for _ in 0..1000{
            let random_string = generate_random_string(10);
            commands.insert(random_string, true);
        }
        commands
    }
    fn list_of_command_channel() -> Vec<(String,String)> {
        vec![(String::from("chloe_rust_dev"),String::from("test")), (String::from("boxbox"),String::from("block")),
        (String::from("itshafu"),String::from("botage")),
        (String::from("chloe_rust_dev"),String::from("botage")),
        (String::from("ratirl"),String::from("chattersview")),
        (String::from("ls"),String::from("chatters")),
        (String::from("chloe_rust_dev16"),String::from("chatterslistview")),
        ]

    }
    #[test]
    #[tracing::instrument]
    fn test_time_hash_map(){
        let channels = create_hashmap_of_commands();
        let list_of_commands = list_of_command_channel();
        for i in 0..100000{
            let random = rand::thread_rng().gen_range(0..list_of_commands.len());
            let (channel, command) = &list_of_commands[random];
            let timer = std::time::Instant::now();
            let _= channels.get(channel).unwrap().get(command).unwrap();
            let elapsed = timer.elapsed();
            println!("Time to get command: {:?} , Requested Command and channel {:?} {:?}, loop iteration {}", elapsed, channel, command, i);
        }
    }
    #[derive(Debug,Clone)]    
    struct Channel {
        name: String,
        commands: Vec<String>
    }
    impl Channel {
        fn new(name: String, commands: Vec<String>) -> Self {
            Channel {
                name,
                commands
            }
        }
    }
        
    fn list_of_command_channel2() -> Vec<(String,String)> {
        vec![(String::from("chloe_rust_dev"),String::from("test")), (String::from("boxbox"),String::from("block")),
        (String::from("maximum"),String::from("botage")),
        (String::from("chloe_rust_dev"),String::from("botage")),
        (String::from("boxbox"),String::from("chattersview")),
        (String::from("nightblue3"),String::from("chatters")),
        (String::from("chloe_rust_dev"),String::from("chatterslistview")),
        ]

    }

    fn create_vector_of_channels()-> Vec<Channel>{
        let mut channels = Vec::new();
        for i in 0..1000{
            if i == 100 {
                channels.push(Channel::new(String::from("chloe_rust_dev"),create_vector_of_commands()));
            } else if i == 200 {
                channels.push(Channel::new(String::from("maximum"),create_vector_of_commands()));
            } else if i == 300 {
                channels.push(Channel::new(String::from("boxbox"),create_vector_of_commands()));
            } else if i == 400 {
                channels.push(Channel::new(String::from("robinsongz"),create_vector_of_commands()));
            } else if i == 500 {
                channels.push(Channel::new(String::from("Emiru"),create_vector_of_commands()));
            } else if i == 600 {
                channels.push(Channel::new(String::from("naowh"),create_vector_of_commands()));
            } else if i == 700 {
                channels.push(Channel::new(String::from("nightblue3"),create_vector_of_commands()));
            } else {
                channels.push(Channel::new(generate_random_string(10),create_vector_of_commands()));
            }
            
        }
        channels
    }

    fn create_vector_of_commands()-> Vec<String>{
        let mut commands = Vec::new();
        commands.push(String::from("test"));
        commands.push(String::from("8ball"));
        commands.push(String::from("accountage"));
        commands.push(String::from("addcom"));
        commands.push(String::from("addquote"));
        commands.push(String::from("addwin"));
        commands.push(String::from("alias"));
        commands.push(String::from("balance"));
        commands.push(String::from("ban"));
        commands.push(String::from("bans"));
        commands.push(String::from("blacklist"));
        commands.push(String::from("blacklistword"));
        commands.push(String::from("blacklistwords"));
        commands.push(String::from("block"));
        commands.push(String::from("blocklist"));
        commands.push(String::from("blocklistword"));
        commands.push(String::from("blocklistwords"));
        commands.push(String::from("blockwords"));
        commands.push(String::from("bonus"));
        commands.push(String::from("bonuspoints"));
        commands.push(String::from("bonuspointsadd"));
        commands.push(String::from("bonuspointsremove"));
        commands.push(String::from("bonusremove"));
        commands.push(String::from("bonuswin"));
        commands.push(String::from("bonuswins"));
        commands.push(String::from("bonuswinsadd"));
        commands.push(String::from("bonuswinsremove"));
        commands.push(String::from("botage"));
        commands.push(String::from("botjoin"));
        commands.push(String::from("botpart"));
        commands.push(String::from("botuptime"));
        commands.push(String::from("chatters"));
        commands.push(String::from("chatterscount"));
        commands.push(String::from("chatterslist"));
        commands.push(String::from("chatterslistadd"));
        commands.push(String::from("chatterslistremove"));
        commands.push(String::from("chatterslisttoggle"));
        commands.push(String::from("chatterslisttoggleadd"));
        commands.push(String::from("chatterslisttoggleremove"));
        commands.push(String::from("chatterslisttoggletoggle"));
        commands.push(String::from("chatterslistview"));
        commands.push(String::from("chattersview"));
        commands
    }
    #[test]
    fn test_time_vector(){
        let channels = create_vector_of_channels();
        let list_of_commands = list_of_command_channel2();
        for i in 0..100000{
            let random = rand::thread_rng().gen_range(0..list_of_commands.len());
            let (channel, command) = &list_of_commands[random];
            let timer = std::time::Instant::now();
            let _= channels.iter().find(|&x| x.name == *channel).unwrap().commands.iter().find(|&x| x == command).unwrap();
            let elapsed = timer.elapsed();
            println!("Time to get command: {:?} , Requested Command and channel {:?} {:?}, loop iteration {}", elapsed, channel, command, i);
        }
    }
}
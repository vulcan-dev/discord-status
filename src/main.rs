use discord_rich_presence::{
    DiscordIpcClient,
    DiscordIpc,
    activity::{Activity, self}
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct Button {
    name: String,
    url: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Config {
    application_id: String,
    details: Option<String>,
    buttons: Vec<Button>,
}

impl Default for Config {
    fn default() -> Self {
        let config = Config {
            application_id: "".to_string(),
            details: None,
            buttons: vec![],
        };

        let mut file = std::fs::File::create("config.json").unwrap();
        serde_json::to_writer_pretty(&mut file, &config).unwrap();
        config
    }
}

fn get_config() -> Config {
    if std::path::Path::new("config.json").exists() {
        let mut file = std::fs::File::open("config.json").unwrap();
        let config: Config = serde_json::from_reader(&mut file).unwrap();
        return config;
    }

    Config::default()
}

fn main() {
    let config = get_config();
    if config.application_id.is_empty() {
        println!("Please set your client ID in config.json");
        std::thread::sleep(std::time::Duration::from_secs(1));
        return;
    }

    if config.buttons.is_empty() {
        println!("Please set your buttons in config.json");
        std::thread::sleep(std::time::Duration::from_secs(1));
        return;
    } else if config.buttons.len() > 2 {
        println!("Only 2 buttons are supported");
        std::thread::sleep(std::time::Duration::from_secs(1));
        return;
    }

    println!("Running Discord Rich Presence, type 'exit' to exit");

    let mut buttons = Vec::new();
    for i in 0..config.buttons.len() {
        buttons.push(activity::Button::new(
            &config.buttons[i].name,
            &config.buttons[i].url,
        ));
    }

    let mut client = DiscordIpcClient::new(&config.application_id)
        .expect("Failed to create Discord client");
    
    client.connect()
        .expect("Failed to connect to Discord");

    // add details if they are set
    if let Some(details) = &config.details {
        client.set_activity(Activity::new()
            .details(&details)
            .buttons(buttons)
        ).expect("Failed to set activity");
    } else {
        client.set_activity(Activity::new()
        .buttons(buttons)
        ).expect("Failed to set activity");
    }

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        if input.trim() == "exit" {
            println!("Exiting");
            break;
        }
    }

    client.close().expect("Failed to close Discord client");
}
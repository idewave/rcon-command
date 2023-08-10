use dotenv::dotenv;
use std::env;
use tungstenite::{connect, Message};
use clap::Parser;
use serde::{Serialize};
use serde_json::Result;
use std::{thread, time::Duration};

/// RCON client for Rust Dedicated Server
#[derive(Parser, Debug)]
struct Args {
    /// Commands to run on server
    #[arg(short, long, num_args=1.., value_delimiter = ',')]
    commands: Vec<String>,

    /// Custom env path (optional)
    #[arg(short, long, default_value="")]
    env_path: String,
}

#[derive(Serialize)]
struct Request {
    identifier: i32,
    message: String,
}

impl Request {
    pub fn new(command: String) -> Self {
        Self {
            identifier: -1,
            message: command,
        }
    }
    pub fn as_message(&self) -> Result<Message> {
        serde_json::to_string(&self).map(Message::text)
    }
}

fn send(url: String, commands: Vec<String>) {
    let (mut socket, _) = connect(url).expect("Cannot connect");

    for command in commands {
        let request = Request::new(command.to_string());
        socket
            .write_message(request.as_message().unwrap())
            .expect(format!("Cannot process command: {}", command).as_str());
    }

    thread::sleep(Duration::from_millis(4000));

    socket.close(None).expect("Cannot close socket");

}

fn main() {
    let Args { commands, env_path } = Args::parse();
    if env_path.is_empty() {
        dotenv().ok();
    } else {
        dotenv::from_path(env_path).ok();
    }

    let server = env::var("SERVER").expect("SERVER must be set");
    let port = env::var("PORT").expect("PORT must be set");
    let password = env::var("PASSWORD").expect("PASSWORD must be set");

    let url = format!("ws://{}:{}/{}", server, port, password);
    send(url, commands);
}

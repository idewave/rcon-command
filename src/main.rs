use dotenv::dotenv;
use clap::{Arg, ArgAction, Command};
use std::env;
use tungstenite::{connect, Message};
use clap::Parser;
use serde::{Serialize};
use serde_json::Result;
// use futures_util::{future, StreamExt};

/// Simple program to greet a person
#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, num_args=1.., value_delimiter = ',')]
    commands: Vec<String>,
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

    socket.close(None).expect("Cannot close socket");

}

fn main() {
    dotenv().ok();

    let server = env::var("SERVER").expect("SERVER must be set");
    let port = env::var("PORT").expect("PORT must be set");
    let password = env::var("PASSWORD").expect("PASSWORD must be set");

    let url = format!("ws://{}:{}/{}", server, port, password);

    let Args { commands } = Args::parse();
    send(url, commands);
}

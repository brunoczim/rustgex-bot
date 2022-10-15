use std::process;

use adapter::telegram::TgMessageChannel;
use app::App;
use commands::help::{HelpCommand, HelpRequestParser};
use config::Config;
use handler::DefaultHandler;

mod future;
mod config;
mod domain;
mod port;
mod adapter;
mod request;
mod command;
mod handler;
mod commands;
mod app;

#[tokio::main]
async fn main() {
    let config = Config::from_env().unwrap_or_else(|error| {
        eprintln!("Error with environment...");
        eprintln!("    {}", error);
        process::exit(1);
    });

    let channel = TgMessageChannel::new(config.token());

    let result = App::new(config)
        .handler(DefaultHandler {
            request_parser: HelpRequestParser,
            command: HelpCommand,
            sender: channel.clone(),
        })
        .run(channel)
        .await;

    if let Err(error) = result {
        eprintln!("Error running application...");
        eprintln!("    {}", error);
        process::exit(1);
    }
}

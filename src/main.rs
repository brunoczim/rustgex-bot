use std::process;

use adapter::telegram::TgMessageChannel;
use app::App;
use commands::help::{HelpCommand, HelpRequestParser};
use env::Environment;
use handler::DefaultHandler;

mod future;
mod env;
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
    let environment = Environment::load().unwrap_or_else(|error| {
        eprintln!("Error with environment...");
        eprintln!("    {}", error);
        process::exit(1);
    });

    let Environment { token, handle } = environment;

    let bot = domain::Bot { handle };
    let channel = TgMessageChannel::new(&token);

    let result = App::new(bot)
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

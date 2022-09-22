pub mod lib;

use clap::Parser;
use inquire::Select;
use lib::{client, server};

#[derive(Parser)]
struct Cli {
    #[clap(short = 's', long = "server")]
    server: Option<bool>,

    #[clap(short = 'p', long = "port")]
    port: Option<String>,
}

fn main() {
    let args = Cli::parse();

    match args.server {
        Some(b) => {
            if b {
                server::run(3);
            } else {
                let port = match args.port {
                    Some(p) => p,
                    None => String::from("3333"),
                };
                client::run(port);
            }
        }
        None => {
            let opt = Select::new("Select one:", vec!["Create a game", "Join a game"])
                .prompt()
                .unwrap_or_default();

            if opt.eq("Create a game") {
                server::run(3);
            } else {
                let port = match args.port {
                    Some(p) => p,
                    None => String::from("3333"),
                };

                client::run(port);
            }
        }
    }
}

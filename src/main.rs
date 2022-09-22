pub mod lib;

use clap::Parser;
use inquire::Select;
use lib::{client, server};

#[derive(Parser)]
struct Cli {
    #[clap(long = "server")]
    server: Option<bool>,

    #[clap(short = 'p', long = "port", default_value = "3333")]
    port: String,

    #[clap(short = 's', long = "size", default_value = "3")]
    board_size: usize,
}

fn main() {
    let args = Cli::parse();

    let run_server = match args.server {
        Some(b) => b,
        None => {
            let opt = Select::new("Select one:", vec!["Create a game", "Join a game"])
                .prompt()
                .unwrap_or_default();

            opt.eq("Create a game")
        }
    };

    if run_server {
        server::run(args.board_size);
    } else {
        client::run(args.port);
    }
}

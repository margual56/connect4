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

    #[clap(short = 's', long = "size", default_value = "4")]
    board_size: usize,

    #[clap(long="ip", default_value="localhost")]
    ip: String,
}

fn main() {
    let args = Cli::parse();

    if args.board_size < 4 || args.board_size > 9 {
        println!("Error: The board size must be [4, 9]");
        std::process::exit(-1);
    }

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
        client::run(args.ip, args.port);
    }
}

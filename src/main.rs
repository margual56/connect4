pub mod lib;

use clap::Parser;
use inquire::Select;
use lib::{client, server};

#[derive(Parser)]
struct Cli {
    #[clap(long = "server")]
    server: Option<bool>,

    #[clap(short = 'p', long = "port")]
    port: Option<String>,

    #[clap(short = 's', long = "size")]
    board_size: Option<usize>,
}

fn main() {
    let args = Cli::parse();
    let board_size = match args.board_size {
        Some(s) => s,
        None => 3,
    };
    let port = match args.port {
        Some(p) => p,
        None => String::from("3333"),
    };

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
        server::run(board_size);
    } else {
        client::run(port);
    }
}

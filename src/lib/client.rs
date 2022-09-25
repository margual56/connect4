use std::io::{Read, Write};
use std::net::TcpStream;

use crate::lib::Board;

pub fn run(port: String) {
    match TcpStream::connect(String::from("localhost:") + &port) {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");

            loop {
                println!("Ready to play, waiting my turn...");

                let mut data = [0 as u8; 1]; // using 6 byte buffer

                while match stream.read_exact(&mut data) {
                    Ok(_) => {
                        match data[0] {
                            0 => println!("Not your turn"),
                            1 => println!("It's your turn!"),
                            2 => println!("Illegal move, please try again"),
                            3 => println!("Game ended"),
                            _ => println!(
                                "Oops! Something went wrong... Code received was '{}'",
                                data[0]
                            ),
                        }

                        data[0] != 1 && data[0] != 2 && data[0] != 3
                    }
                    Err(e) => {
                        println!("Failed to receive data: {}", e);

                        true
                    }
                } {}

                if data[0] == 3 {
                    println!("Game ended!");
                    return;
                }

                // 9*9
                let mut data2 = [0 as u8; 81];

                match stream.read(&mut data2) {
                    Ok(size) => {
                        println!(
                            "\n{}",
                            Board::to_string_from_bytes(
                                data2.to_vec(),
                                (size as f64).sqrt().floor() as usize
                            )
                        );
                    }
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                    }
                }

                println!("\n");

                let col = inquire::Text::new(
                    "Where to drop the chip? (column number starting from zero): ",
                )
                .prompt()
                .unwrap_or(String::from("0"))
                .parse::<u8>()
                .unwrap_or(0);

                stream.write_all(&[col]).unwrap();

                println!("Sent!");

                match stream.read(&mut data2) {
                    Ok(size) => {
                        if size == 1 {
                            println!("\tSomething went wrong");
                            println!("\tIn an ideal world, you would be able to retry");
                            println!("\tBut for now, it is a pending feature");
                        } else {
                            println!(
                                "\n{}",
                                Board::to_string_from_bytes(
                                    data2.to_vec(),
                                    (size as f64).sqrt().floor() as usize
                                )
                            );
                        }
                    }
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}

use crate::lib::Chip;

use super::Board;
use std::io::{Error, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};

pub fn run(port: String, size: usize) {
    let listener = TcpListener::bind(String::from("0.0.0.0:") + &port).unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port {}", port);

    let board = &mut Board::new(size);
    let mut player1 = next_player(&listener);
    let mut player2 = next_player(&listener);

    println!("Connected both players successfully!");
    println!("The game will start now, player 1 goes first with color Yellow");

    let mut winner: Option<Chip> = None;
    let mut player1_turn = true;
    while winner.is_none() {
        println!("Turn {}", board.moves);
        println!("{}", board.to_string());
        if player1_turn {
            winner = handle_turn(board, &player1, Chip::YELLOW);
        } else {
            winner = handle_turn(board, &player2, Chip::RED);
        }

        println!("Winner is {:?}", winner);

        player1_turn = !player1_turn;
    }

    print!("Telling player1 that game has ended... ");
    if let Some(w) = winner {
        if w == Chip::YELLOW {
            player1.write(&[4 as u8]).unwrap();
        }else{
            player1.write(&[3 as u8]).unwrap();
        }
    }else{
        player1.write(&[255 as u8]).unwrap();
    }
    println!("Done!");
    print!("Telling player2 that game has ended... ");
    if let Some(w) = winner {
        if w == Chip::RED {
            player2.write(&[4 as u8]).unwrap();
        }else{
            player2.write(&[3 as u8]).unwrap();
        }
    }else{
        player2.write(&[255 as u8]).unwrap();
    }
    println!("Done!");

    println!("Shutting down the server...");
    // close the socket server
    drop(listener);
}

fn handle_turn(board: &mut Board, mut stream: &TcpStream, c: Chip) -> Option<Chip> {
    print!("Letting player know that it's its turn (sending a 1)... ");
    stream.write(&[1 as u8]).unwrap();

    println!("Done!");
    print!("Sending player the board... ");

    stream.write(&board.as_1d()).unwrap();

    println!("Done!");
    print!("Waiting for player move... ");

    let mut data = [1 as u8; 1];
    while match stream.read(&mut data) {
        Ok(size) => {
            println!("Received {} bytes", size);

            let result: usize = data[0] as usize;

            println!(
                "After conversion to usize, the received message was: {}",
                result
            );

            let retry: bool = match board.drop_chip(result, c) {
                Ok(r) => {
                    stream.write(&board.as_1d()).unwrap();

                    return r;
                }
                Err(e) => {
                    println!("{}", e);
                    println!("Sending signal to try again:");

                    print!("\tLetting player know that it has to retry (sending a 2)... ");
                    stream.write(&[2 as u8]).unwrap();

                    println!("Done!");
                    print!("Waiting for player move... ");

                    true
                }
            };

            !retry
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}

    return None;
}

fn next_player(listener: &TcpListener) -> TcpStream {
    loop {
        match get_next_client(listener) {
            Ok(c) => {
                return c;
            }
            Err(e) => {
                println!("Error connecting to player: {}", e);
            }
        }
    }
}

fn get_next_client(listener: &TcpListener) -> Result<TcpStream, Error> {
    match listener.incoming().next().unwrap() {
        Ok(stream) => {
            println!("New connection: {}", stream.peer_addr().unwrap());

            return Ok(stream);
        }
        Err(e) => {
            println!("Error: {}", e);
            return Err(e);
        }
    }
}

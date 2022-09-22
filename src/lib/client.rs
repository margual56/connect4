use std::io::{Read, Write};
use std::net::TcpStream;

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

                // FIXME: The formula is incorrect
                // Size of the board representation in bytes
                // Each cell has its borders (+1) and surrounding spaces (+2), so
                // Each cell has 4 characters wide and 4 characters tall (-1 repeated character), so
                // The board has size*size cells. Therefore the size is size*size*(4*4 - 1) = size*size*15, but
                // We need to add the final border (+size*2)
                const MAX_BOARD_SIZE: usize = 9;
                let mut data2 =
                    [0 as u8; MAX_BOARD_SIZE * MAX_BOARD_SIZE * 30 + MAX_BOARD_SIZE * 2];

                match stream.read(&mut data2) {
                    Ok(size) => {
                        println!("\n{}", String::from_utf8_lossy(&data2[0..size]));
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
                        println!("\n{}", String::from_utf8_lossy(&data2[0..size]));
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

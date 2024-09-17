mod cli;
use std::io::{stdin, BufRead};

use cli::Args;

use clap::Parser;
use decode::{decode_connection, decode_game};

fn main() {
    let Args {
        protocol,
        hex,
        stdin: _,
    } = Args::parse();

    let bytes: Vec<u8> = {
        match hex {
            Some(hex_str) => hex::decode(hex_str).expect("Invalid hexstream"),
            None => {
                let mut input = String::new();

                match stdin().lock().read_line(&mut input) {
                    Ok(_) => hex::decode(input.trim_end()).expect("Invalid hexstream"),
                    Err(err) => {
                        panic!("Failed to read stdin: {}", err);
                    }
                }
            }
        }
    };

    match protocol {
        cli::Protocol::Game => decode_game(bytes).iter().for_each(|r| match r {
            Ok(m) => println!("{m}"),
            Err(e) => eprintln!("{e}"),
        }),
        cli::Protocol::Connection => decode_connection(bytes).iter().for_each(|r| match r {
            Ok(m) => println!("{m}"),
            Err(e) => eprintln!("{e}"),
        }),
    }
}

/* Copyright (c) 2021 Matthew Carr. Licensed under the GNU GPL version 3. */

use std::env;
use std::io;
use std::io::Result;
use std::net::{TcpListener, TcpStream};

const DEFAULT_PORT: u16 = 41270;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    match parse_command_line(args) {
        Command::Client(host, port) => run_client(host, port),
        Command::Server(host, port) => run_server(host, port)
    }
}

#[derive(Debug)]
enum Command {
    Client(String, u16),
    Server(String, u16)
}

fn parse_command_line(mut args: Vec<String>) -> Command {
    if 0 == args.len() {
        panic!("Too few command line arguments provided.");
    }
    let last = args.pop().unwrap();
    let (host, port) = parse_host_and_port(last);
    match args.pop().as_deref() {
        Some("-l") => Command::Server(host, port),
        _ => Command::Client(host, port)
    }
}

fn parse_host_and_port(arg: String) -> (String, u16) {
    match arg.find(':') {
        Some(index) =>
            (arg[..index].to_string(), arg[(index + 1)..].parse::<u16>().unwrap()),
        None => (arg, DEFAULT_PORT)
    }
}

fn run_server(host: String, port: u16) -> Result<()> {
    let listener = TcpListener::bind((host, port))?;

    for result in listener.incoming() {
        match result {
            Ok(stream) => {
                handle_client(stream)?;
                break;
            }
            Err(error) => {
                println!("{:?}", error);
                continue;
            }
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> Result<()> {
    let mut stdout = io::stdout();
    io::copy(&mut stream, &mut stdout)?;
    Ok(())
}

fn run_client(host: String, port: u16) -> Result<()> {
    let mut stream = TcpStream::connect((host, port))?;
    let mut stdin = io::stdin();
    io::copy(&mut stdin, &mut stream)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command_line_server() {
        let args = vec!["-l".to_string(), "localhost:1337".to_string()];

        match parse_command_line(args) {
            Command::Server(host, port) => {
                assert_eq!("localhost", host);
                assert_eq!(1337, port);
            },
            _ => panic!("parse_command_line has a bug!")
        }
    }

    #[test]
    fn test_parse_command_line_client() {
        let args = vec!["example.com:4221".to_string()];

        match parse_command_line(args) {
            Command::Client(host, port) => {
                assert_eq!("example.com", host);
                assert_eq!(4221, port);
            },
            _ => panic!("parse_command_line has a bug!")
        }
    }

    #[test]
    fn test_parse_command_line_server_default_port() {
        let args = vec!["-l".to_string(), "localhost".to_string()];

        match parse_command_line(args) {
            Command::Server(host, port) => {
                assert_eq!("localhost", host);
                assert_eq!(DEFAULT_PORT, port);
            },
            _ => panic!("parse_command_line has a bug!")
        }
    }
}

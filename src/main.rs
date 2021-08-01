/* Copyright (c) 2021 Matthew Carr. Licensed under the GNU GPL version 3. */

use std::env;
use std::fmt::Display;
use std::io;
use std::net::{TcpListener, TcpStream};

const DEFAULT_PORT: u16 = 41270;

type Result<T> = std::result::Result<T, String>;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    match Command::new(args)? {
        Command::Client(host, port) => run_client(host, port),
        Command::Server(host, port) => run_server(host, port)
    }
}

#[derive(Debug)]
enum Command {
    Client(String, u16),
    Server(String, u16)
}

impl Command {
    fn new(mut args: Vec<String>) -> Result<Command> {
        if 0 == args.len() {
            return Err("Too few command line arguments provided".to_string());
        }
        let last = args.pop().ok_or("Could not get the last command line argument")?;
        let (host, port) = Command::parse_host_and_port(last)?;
        match args.pop().as_deref() {
            Some("-l") => Ok(Command::Server(host, port)),
            _ => Ok(Command::Client(host, port))
        }
    }
    
    fn parse_host_and_port(arg: String) -> Result<(String, u16)> {
        match arg.find(':') {
            Some(index) => {
                let port = arg[(index + 1)..].parse::<u16>()
                    .or_else(format_error)?;
                Ok((arg[..index].to_string(), port))
            },
            None => Ok((arg, DEFAULT_PORT))
        }
    }
}

fn run_server(host: String, port: u16) -> Result<()> {
    let listener = TcpListener::bind((host, port))
        .or_else(format_error)?;

    for result in listener.incoming() {
        match result {
            Ok(stream) => {
                handle_client(stream)?;
                break;
            }
            Err(error) => {
                eprintln!("{:?}", error);
                continue;
            }
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> Result<()> {
    let mut stdout = io::stdout();
    io::copy(&mut stream, &mut stdout).or_else(format_error)?;
    Ok(())
}

fn run_client(host: String, port: u16) -> Result<()> {
    let mut stream = TcpStream::connect((host, port)).or_else(format_error)?;
    let mut stdin = io::stdin();
    io::copy(&mut stdin, &mut stream).or_else(format_error)?;
    Ok(())
}

/// Formats the given error as a String.
fn format_error<T, E: Display>(err: E) -> Result<T> {
    Err(format!("{}", err))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command_line_server() {
        let args = vec!["-l".to_string(), "localhost:1337".to_string()];

        match Command::new(args).unwrap() {
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

        match Command::new(args).unwrap() {
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

        match Command::new(args).unwrap() {
            Command::Server(host, port) => {
                assert_eq!("localhost", host);
                assert_eq!(DEFAULT_PORT, port);
            },
            _ => panic!("parse_command_line has a bug!")
        }
    }
}

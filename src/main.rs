use std::env;

const DEFAULT_PORT: u16 = 41270;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match parse_command_line(args) {
        Command::Client(host, port) => println!("Client: '{}':'{}'", host, port),
        Command::Server(host, port) => println!("Server: '{}':'{}'", host, port)
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_command_line_server() {
        let args = vec!["-l".to_string(), "localhost:1337".to_string()];

        match super::parse_command_line(args) {
            super::Command::Server(host, port) => {
                assert_eq!("localhost", host);
                assert_eq!(1337, port);
            },
            _ => panic!("parse_command_line has a bug!")
        }
    }

    #[test]
    fn test_parse_command_line_client() {
        let args = vec!["carrfound.org:4221".to_string()];

        match super::parse_command_line(args) {
            super::Command::Client(host, port) => {
                assert_eq!("carrfound.org", host);
                assert_eq!(4221, port);
            },
            _ => panic!("parse_command_line has a bug!")
        }
    }

    #[test]
    fn test_parse_command_line_server_default_port() {
        let args = vec!["-l".to_string(), "localhost".to_string()];

        match super::parse_command_line(args) {
            super::Command::Server(host, port) => {
                assert_eq!("localhost", host);
                assert_eq!(super::DEFAULT_PORT, port);
            },
            _ => panic!("parse_command_line has a bug!")
        }
    }
}

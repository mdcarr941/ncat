use std::env;

const DEFAULT_PORT: u16 = 41270;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match parse_command_line(args) {
        Command::Client(host, port) => println!("Client: '{}':'{}'", host, port),
        Command::Server(host, port) => println!("Server: '{}':'{}'", host, port)
    }
}

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

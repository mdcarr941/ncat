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

fn parse_command_line(args: Vec<String>) -> Command {
    if 0 == args.len() {
        panic!("Too few command line arguments provided.");
    }
    let (host, port) = parse_host_and_port(
        if args.len() > 1 { &args[1] } else { &args[0] });
    if args[0] == "-l" {
        Command::Server(host, port)
    }
    else {
        Command::Client(host, port)
    }
}

fn parse_host_and_port(arg: &String) -> (String, u16) {
    let split: Vec<&str> = arg.split(':').collect();
    let first = split.first().unwrap();
    let port = if split.len() > 1
        { split.last().unwrap().parse::<u16>().unwrap() }
        else { DEFAULT_PORT };
    (first.to_string(), port)
}

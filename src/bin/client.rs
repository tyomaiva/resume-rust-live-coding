use clap::{load_yaml, App};
use serde::Serialize;
use std::io::prelude::*;
use std::net::TcpStream;

#[derive(Serialize)]
struct Employee {
    name: String,
    position: String,
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();

    let name = matches.value_of("name").unwrap().to_string();
    let position = matches.value_of("position").unwrap().to_string();

    let destination = "127.0.0.1:9000";
    let mut stream =
        TcpStream::connect(destination).expect(&format!("Could not connect to {}", destination));
    let payload = Employee {
        name: name,
        position: position,
    };
    let serialized = serde_json::to_vec(&payload).expect("Could not serialize the input data!");
    stream
        .write(&serialized)
        .expect("Couldn't send the message!");
    println!("The message is sent!");
}

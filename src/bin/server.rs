//! Back-end server application that updates the database of employees.

use std::net::TcpListener;
use std::sync::mpsc::channel;
use std::io::Read;
use std::collections::HashMap;

use threadpool::ThreadPool;
use serde::Deserialize;

/// Encodes the employee data like the position
#[derive(PartialEq, Debug, Deserialize)]
struct Employee {
    name: String,
    position: String,
}

/// Parse the JSON request and send it further to database
fn execute_request(received_bytes: Vec<u8>, db_sender: std::sync::mpsc::Sender<Employee>) {
    let received_string = std::str::from_utf8(&received_bytes).unwrap();
    let e: Employee = serde_json::from_str(received_string).unwrap();
    db_sender.send(e).unwrap();
}

/// Update the database with a new employee
fn update_database(receiver: std::sync::mpsc::Receiver<Employee>) {
    let mut state = HashMap::new();
    loop {
        let new_emp = receiver.recv().unwrap();
        state.insert(new_emp.name.clone(), new_emp);
        println!("Database state: {:?}", state);
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9000").expect("Cannot listen to the port");

    let pool = ThreadPool::new(8);
    let (sender, receiver) = channel::<Employee>();

    // Database thread
    pool.execute(move || update_database(receiver));

    let mut buffer = [0u8; 2048];
    for stream_res in listener.incoming() {
        match stream_res {
            Ok(mut stream) => {
                println!("New client is accepted!");
                let message_size = stream.read(&mut buffer).unwrap();
                let received = buffer[..message_size].to_vec();
                let sender_copy = sender.clone();
                pool.execute(move || execute_request(received, sender_copy));
            }
            Err(e) => {
                println!("Reception failed: {:?}", e);
            }
        }
    }
    println!("Finished.");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_request() {
        let (sender, receiver) = channel::<Employee>();
        execute_request(
            "{\"name\": \"my-name\", \"position\":\"my-position\"}"
                .as_bytes()
                .to_vec(),
            sender,
        );
        assert_eq!(
            receiver.recv().unwrap(),
            Employee {
                name: String::from("my-name"),
                position: String::from("my-position"),
            }
        );
    }
}
//! Back-end server application that updates the database of employees.

use std::collections::HashMap;
// use std::io::Read;
use std::sync::mpsc::channel;

use serde::Deserialize;
// use threadpool::ThreadPool;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

/// Encodes the employee data like the position
#[derive(PartialEq, Debug, Deserialize)]
struct Employee {
    name: String,
    position: String,
}

/// Parse the JSON request and send it further to database
// fn execute_request(received_bytes: Vec<u8>, db_sender: std::sync::mpsc::Sender<Employee>) {
//     let received_string = std::str::from_utf8(&received_bytes).unwrap();
//     let e: Employee = serde_json::from_str(received_string).unwrap();
//     db_sender.send(e).unwrap();
// }

/// Update the database with a new employee
// fn update_database(receiver: std::sync::mpsc::Receiver<Employee>) {
//     let mut state = HashMap::new();
//     loop {
//         let new_emp = receiver.recv().unwrap();
//         state.insert(new_emp.name.clone(), new_emp);
//         println!("Database state: {:?}", state);
//     }
// }

//(flavor = "multi_thread", worker_threads = 10)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut listener = TcpListener::bind("127.0.0.1:9000").await?;

    // let pool = ThreadPool::new(8);
    // let (sender, receiver) = channel::<Employee>();

    // // Database thread
    // pool.execute(move || update_database(receiver));

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = [0; 1024];
        })
        }
    }

    // let mut buffer = [0u8; 2048];
    // for stream_res in listener.incoming() {
    //     match stream_res {
    //         Ok(mut stream) => {
    //             println!("New client is accepted!");
    //             let message_size = stream.read(&mut buffer).unwrap();
    //             let received = buffer[..message_size].to_vex();
    //             let sender_copy = sender.clone();
    //             pool.execute(move || execute_request(received, sender_copy));
    //         }
    //         Err(e) => {
    //             println!("Reception failed: {:?}", e);
    //         }
    //     }
    // }
    // println!("Finished.");
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let listener = TcpListener::bind("127.0.0.1:8080").await?;

//     loop {
//         // let (mut socket, _) = listener.accept().await?;

//         // tokio::spawn(async move {
//         //     let mut buf = [0; 1024];

//         //     // In a loop, read data from the socket and write the data back.
//         //     loop {
//         //         let n = match socket.read(&mut buf).await {
//         //             // socket closed
//         //             Ok(n) if n == 0 => return,
//         //             Ok(n) => n,
//         //             Err(e) => {
//         //                 eprintln!("failed to read from socket; err = {:?}", e);
//         //                 return;
//         //             }
//         //         };

//         //         // Write the data back
//         //         if let Err(e) = socket.write_all(&buf[0..n]).await {
//         //             eprintln!("failed to write to socket; err = {:?}", e);
//         //             return;
//         //         }
//         //     }
//         // });
//     }
// }

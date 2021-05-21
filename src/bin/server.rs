use std::net::TcpListener;
use std::sync::mpsc::channel;
use std::io::Read;

use threadpool::ThreadPool;


/// Encodes the employee data like the position
struct Employee {
    name: String,
    position: String,
}

/// Parse the JSON request and send it further to database
fn execute_request(received_bytes: Vec<u8>, db_sender: std::sync::mpsc::Sender<Employee>) {
    println!("{:?}", received_bytes);
}


fn main() {
    let listener = TcpListener::bind("127.0.0.1:9000").expect("Cannot listen to the port");

    let pool = ThreadPool::new(8);
    let (sender, receiver) = channel::<Employee>();

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

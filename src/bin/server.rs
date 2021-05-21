use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9000").expect("Cannot listen to the port");
    for stream_res in listener.incoming() {
        match stream_res {
            Ok(mut stream) => {
                println!("New client is accepted!");
            }
            Err(e) => {
                println!("Reception failed: {:?}", e);
            }
        }
    }
    println!("Finished.");
}

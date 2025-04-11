// importing need modules
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream){
    // this is a buffer to read data from the client
    let mut buffer =[0; 1024];
    // this line reads data from the stream anbd stores it in the buffer.
    stream.read(&mut buffer).expect("Failed to read from client!");
    // this line converts the data in the buffer into a utf-8 enccoded string.
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request: {}", request);
    let response = "hello, client!".as_bytes();
    stream.write(response).expect("Failed to write response");
}

// entry point
fn main(){
    let listener = TcpListener::bind("127.0.0.1:8080").
    expect("Failed to bind address");
    println!("Server listening on 127.0.0.1:8080");

    for stream in listener.incoming(){
        match stream{
            Ok(stream)=> {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) =>{
                eprintln!("failed to connect to client: {}", e);
                //stderr=stardard error stream
            }
        }
    }
}

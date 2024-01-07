use std::{
    io::{Read, Write},
    net::{TcpListener,TcpStream}

};

fn handle_client(mut stream: TcpStream) {
    // buffer needed to read data from client
    let mut buffer = [0;1024];


    //line reads data from the stream and stores it in a buffer
    stream.read(&mut buffer).expect("Failed to read from
     client");
    
    //line converts the data in the buffer into a utf 8 enccoded strings.
    let request = String::from_utf8_lossy(&buffer[..]);
    let response = "Hello, Client!".as_bytes();
    stream.write(response).expect("failed to write response!");



}


fn main() {
    let listener = TcpListener::bind("127.0.0.1;8080").
    expect("Failed to bind to address");
    println!("server listening on 127.0.0.1:8080");

    for stream in listener.incoming(){
        match stream{
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {

                eprintln!("failed to establish connection:{}",e);

            }
        }
    }

}

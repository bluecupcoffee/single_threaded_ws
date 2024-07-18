use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    process,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878");
    if let Ok(tl) = listener {
        println!("Listener succesfully created");
        for stream in tl.incoming() {
            let stream = stream.unwrap();

            handle_connection(stream);
        }
    } else if let Err(e) = listener {
        eprintln!("Error when attempting to bind to address: {:?}", e);
        process::exit(1);
    }

    println!("Hello, world!");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    let stream_r = stream.write_all(response.as_bytes());
    match stream_r {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error! {:#?}", e);
        }
    }
}

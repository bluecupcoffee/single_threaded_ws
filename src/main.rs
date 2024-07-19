use std::{fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, process};

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
    println!("{:#?}", http_request);
    let request_line = http_request.iter().next().unwrap();
    let(status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();

}


// response format

// HTTP-Version Status-Code Reason-Phrase CRLF
// headers CRLF
// message-body

// request format

// Method Request-URI HTTP-Version CRLF
// headers CRLF
// message-body
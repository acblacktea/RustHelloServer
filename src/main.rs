use std::{net::{TcpListener, TcpStream}, io::{prelude::*, BufRead}, fs, thread};
use std::fmt::format;
use std::io::BufReader;
use std::thread::Thread;
use std::time::Duration;
use helloSever::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection2(stream);
        });
    }
}


fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("./src/hello.html").unwrap();
    let length = contents.len();
    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}

fn handle_connection2(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line_result = buf_reader.lines().next();
    let request_line: String;
    if request_line_result.is_some() {
        request_line = request_line_result.unwrap().unwrap()
    } else {
        return;
    }

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "./src/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "./src/hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "./src/404.html")
    };

    println!("request_line {request_line} status line {status_line}, filename {filename}");

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
}
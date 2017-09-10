
use std::io;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;
use std::process;

fn send(path: &str) -> Result<(), io::Error> {
    // TODO
    let mut f = OpenOptions::new()
        .read(true)
        .truncate(false)
        .open(path)?;

    let mut buf: Vec<u8> = Vec::new();
    f.read_to_end(&mut buf)?;
    Ok(())
}

pub fn host(address: &str, port: &str) {
    let listener = TcpListener::bind([address,":",port].concat()).unwrap();
    println!("server: ready");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        thread::spawn(move || {
            handle_client(stream);
        });
    }
    println!("server: shutting down");
}

fn handle_client(mut stream: TcpStream) {
    println!("server: connected {:?}", stream.peer_addr().unwrap());

    let mut string;
    let mut c: usize;
    loop {
        let res = match super::read_socket(&mut stream, 1) {
            Ok(t) => t,
            Err(e) => {
                notify_client_err(&mut stream,e);
                continue
            },
        };
        string = res.0;
        c = res.1;
        break;
    }

    println!("server: received {}b read as: '{}'", c, string);

    match action(&mut stream, string.as_str()) {
        Ok(_) => (),
        Err(e) => notify_client_err(&mut stream, e),

    };
}

fn notify_client_err(stream: &mut TcpStream, error: io::Error) {
    eprintln!("server: Unable handle request. threw '{}'", error);
    stream.write_all(b"Err");
}

fn action(stream: &mut TcpStream, input: &str) -> Result<(), io::Error>  {
    match input {
        "ask" => send_ask(stream),
        _ => super::send_file(stream, input)?,
    };
    Ok(())
}

fn send_ask(stream: &mut TcpStream) {
    let mut files = "examples/files/foo.txt";
    super::encode("ask", &mut files.to_string(), "");
}
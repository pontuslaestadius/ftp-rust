
use std::io;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;
use std::process;

pub fn host(address: &str, port: &str) {
    let listener = TcpListener::bind([address,":",port].concat()).unwrap();
    println!("server: ready");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        thread::spawn(move || {
            handle_client(stream);
        });
    }
    println!("shutting down");
}

fn handle_client(mut stream: TcpStream) {
    println!("connected {:?}", stream.peer_addr().unwrap());

    let mut string: String = String::new();
    let mut c: usize;

    let string = match super::read_socket(&mut stream, 2000) {
        Ok(t) => t,
        Err(e) => {
            notify_client_err(&mut stream,e);
            process::exit(429);
        },
    };

    println!("server: received {}b => '{}'", string.len(), string);

    match action(&mut stream, string.as_str()) {
        Ok(_) => (),
        Err(e) => notify_client_err(&mut stream, e),
    };
}

fn notify_client_err(stream: &mut TcpStream, error: io::Error) {
    eprintln!("server: Unable handle request. threw '{}'", error);

    let fields = vec!("name", "type");
    let values = vec!("err", "disp");
    let meta = super::metadata::new(fields, values);
    let mut cont = error.to_string();
    // Catching errors here would only serve as a debugging tool.
    match super::encode::from_meta_data(meta, &mut cont) {
        Ok(_) => (),
        Err(e) => eprintln!("error was thrown when notifying client '{}'", e),
    };
}

fn action(stream: &mut TcpStream, input: &str) -> Result<(), io::Error>  {
    let mut command = input.split(' ');
    match command.next().unwrap() {
        "ask" => super::send_ask(stream),
        "get" => super::send_get(stream, command.next().unwrap()),
        _ => {
            println!("unknown action");
            Ok(())
        },
    }
}
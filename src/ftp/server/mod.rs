
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
    println!("server: shutting down");
}

fn handle_client(mut stream: TcpStream) {
    println!("server: connected {:?}", stream.peer_addr().unwrap());

    let mut string;
    let mut c: usize;
    loop {
        let res = match super::read_socket(&mut stream, 100000) {
            Ok(t) => t,
            Err(e) => {
                //notify_client_err(&mut stream,e);
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
    match &input[0..3] {
        "ask" => super::send_ask(stream),
        "get" => super::send_get(stream, &input[4..]),
        _ => {
            println!("unknown action");
            Ok(()) // TODO should this be OK?
        },
    }
}
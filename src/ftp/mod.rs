use std::net::TcpStream;
use std::io::prelude::*;
use std::io;
use std::fs::{File, OpenOptions};
use std::time;
use std::io::{Error, ErrorKind};

pub mod server;

use std::thread;

const STDBUF: usize = 8154;

pub fn byte_to_string(bytes: &[u8], range: usize) -> String {
    let mut string = String::new();
    for i in 0..range {
        string.push(bytes[i] as char);
    }
    string
}

fn encode_file(mut file: File, title: &str) -> Vec<String> {
    let mut content: String = String::new();
    file.read_to_string(&mut content);
    encode("file", &mut content, title)
}

fn encode(encoded_type: &str, mut content: &mut String, title: &str) -> Vec<String> {
    let stdbuf = STDBUF-154;

    let size = content.len();
    let mut split_at = 0;

    let mut packages = Vec::new();


    let mut pktnr = 1; // Meta data depicting which packet this is. // TODO should this start at 0?
    while split_at != content.len() {
        pktnr += 1;

        match (STDBUF + split_at) > content.len() {
            true => split_at = content.len(),
            false => split_at += stdbuf,
        };

        let min = match (content.len() as i16-split_at as i16 -stdbuf as i16) > 0 {
            true => split_at-stdbuf,
            false => 0,
        };

        println!("content {} split_at {} min {}", content.len(), split_at, min);

         // TODO make this process more modular.
        packages.push([
            "{\
                meta\
                    {\
                        type:", encoded_type, ";\
                        pktnr:", pktnr.to_string().as_str(), ";\
                        name:", title, ";\
                        size:", content[min..split_at].as_ref(),
            ";}\
             }\
                ", format_tag("cont", content[min..split_at].as_ref()).as_str() , "\
             }"
        ].concat());
    };
    packages
}
/*

fn format_meta_data (fields: [&str], values: [&str]) -> &str {

}
*/

fn format_tag<'a, 'b> (tag: &'a str, cont: &'a str) -> String {
    [tag, "{", cont, "}"].concat()
}

fn send_file(stream: &mut TcpStream, path: &str) -> Result<(), io::Error> {
    let content = get_file(path)?;
    for item in content {
        stream.write_all(item.as_bytes());
    }
    Ok(())
}

fn get_file(path: &str) -> Result<Vec<String>, io::Error> {
    let mut f = OpenOptions::new()
        .read(true)
        .truncate(false)
        .open(path)?;

    let path_string = path.to_string();

    let mut str: &str = "undefined.txt";

    // Splits the path by directory dividers so the
    // last split in the path is the file name.
    let split = path_string.split('/');
    for s in split {
        str = s;
    }

    let encoded = encode_file(f, str);
    println!("server: sending {}b", encoded.len());
    Ok(encoded)
}

pub fn start_server(address: &str, port: &str) {
    thread::spawn(move|| {server::host("127.0.0.1", "19005");});
}

fn private_client() {
    let address = "127.0.0.1:19005";
    let mut stream = match connect(address) {
        Ok(t)  => t,
        Err(e) => {
            eprintln!("client: caught error while connecting, {}", e);
            panic!("client: caught error while connecting, {}", e);  // TODO bad practice to panic here.
        },
    };
    println!("client: connected to {:?}", stream.peer_addr().unwrap());

    let path = "examples/files/foo.txt"; // TODO user sent information
    println!("client: i want '{}'", path);
    match get(&mut stream, path) {
        Ok(_) => (),
        Err(e) => panic!("unable to process get request. threw '{}'", e),
    };
}

pub fn get(mut stream: &mut TcpStream, path: &str) -> Result<(), io::Error> {
    let delay = time::Duration::from_millis(120);
    thread::sleep(delay);
    receive(&mut stream, path)?;
    Ok(())
}

pub fn start_client() {
    thread::spawn(|| {private_client();});
}

// Connects through TCP and return the stream.
pub fn connect(address: &str) -> Result<TcpStream, io::Error> { //TODO
    let stream = TcpStream::connect(address)?;
    Ok(stream)
}

// Returns a Buffer with the content of the file.
pub fn get_buffer(path: &str) -> Result<Buffer, io::Error> {
    let mut f = OpenOptions::new()
        .read(true)
        .truncate(false)
        .open(path)?;

    let mut buf: Vec<u8> = Vec::new();
    f.read_to_end(&mut buf)?;
    Ok(Buffer::new(buf))
}

// Writes all the content from the buffer to the stream.
pub fn send(stream: &mut TcpStream, buf: &mut Buffer) -> Result<(), io::Error> {
    stream.write_all(buf.get())?;
    Ok(())
}

pub fn receive(mut stream: &mut TcpStream, path: &str) -> Result<(), io::Error>{
    let _ = stream.write_all(path.as_bytes());
    //println!("client: waiting for response from {}...", stream.peer_addr()?);

    let (string, c) = read_socket(&mut stream, 5)?;
    println!("client: received {}b", c);

    decode_file(string)?;
    Ok(())
}

pub fn read_socket<'a>
(stream: &mut TcpStream, timeout_sec: usize) -> Result<(String, usize), io::Error> {
    let mut buffer = [0; STDBUF]; // TODO improve length
    let mut tries = 0;
    let mut c;
    let increment_delay = 250;
    let timeout = (timeout_sec as f64/(increment_delay as f64*0.001)) as usize;
    let delay = time::Duration::from_millis(increment_delay);
    stream.set_read_timeout(Some(delay))?;
    loop {
        c = stream.read(&mut buffer[..])?;
        if c != 0 {
            break;
        }

        tries += 1;
        if tries > timeout {
            return Err(Error::new(ErrorKind::Other, "timeout"));
        }
    };
    Ok((byte_to_string(&buffer, c), c))
}

pub fn decode_file(string: String) -> Result<File, io::Error> {
    let mut split = string.split('{');

    let mut decoded_type: &str = "";
    let mut decoded_name: &str = "undefined.txt";
    let mut decoded_cont: &str = "";
    let mut decoded_size: &str = "";

    let mut section = split.next().unwrap();
    while section == "" {
        section = split.next().unwrap();
    }

    if section == "Err" {
        panic!("received an error from the server");
    }

    if section == "meta" {
        let mut section2 = split.next().unwrap().split('}');
        let mut row = section2.next().unwrap().split(';');
        for each in row {
            if each == "" {continue};
            let mut i = each.split(":");

            let property = i.next().unwrap();
            let value = i.next().unwrap();
            match property  {
                "name" => decoded_name = value,
                "type" => decoded_type = value,
                "size" => decoded_type = value,
                _ => println!("unknown meta data '{}' containing '{}'", property, value),
            };
        } // Handle packets that don't start with 'meta'

        {
            decoded_cont = split.next().unwrap();
            let len = decoded_cont.len() -2;
            decoded_cont = &decoded_cont[..len];
        }

        println!("name '{}' \t type '{}' \t size '{}b'", decoded_name,decoded_type,decoded_cont.len());
    }

    let path = ["examples/", decoded_name].concat();
    let mut f = File::create(path)?;
    f.write_all(decoded_cont.as_bytes());

    Ok(f)
}

pub fn progress_bar(buf: Buffer) {
    // TODO (Optional)
    // Displays the progress of the download in the terminal.
    // Example:
    // overview:
    // size: 30KB,
    // elapse: 2s,
    // speed: 15KB/s,
}

pub struct Buffer {
    buf: Vec<u8>,
}

impl Buffer {
    pub fn new(buf: Vec<u8>) -> Buffer {
        Buffer {
            buf
        }
    }

    pub fn get(&mut self) -> &mut Vec<u8> {
        &mut self.buf
    }
}
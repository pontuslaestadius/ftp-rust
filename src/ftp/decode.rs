use std::net::TcpStream;
use std::io::prelude::*;
use std::io;
use std::fs::{File, OpenOptions};

// Will decode a string in to a file format.

// TODO make a safe and unsafe version.
// Safe should prompt the user if it wants to save the file or not.
pub fn file(string: String) -> Result<File, io::Error> {
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
            if each == "" {continue}; // Empty rows are ignored.
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

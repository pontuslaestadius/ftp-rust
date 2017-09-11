use std::net::TcpStream;
use std::io::prelude::*;
use std::io;
use std::fs::{File, OpenOptions};

pub fn generic(string: String) -> Result<(), io::Error> {

    println!("generic decoding on '{}'",string);

    let mut split = string.split('{');

    let mut decoded_type: &str = "";
    let mut decoded_name: &str = "undefined.txt";
    let mut decoded_cont: &str = "";
    let mut decoded_size: &str = "";
    let mut decoded_pktn: &str = "";

    let mut section = split.next().unwrap();
    while section == "" {
        section = split.next().unwrap();
    }

    // The decoding of meta data should always be the same,
    // so this rigid implementation should work for all cases hopefully.
    if section == "meta" {
        let mut section2 = split.next().unwrap().split('}');
        let mut rows = section2.next().unwrap().split(';');
        for each in rows {
            if each == "" {continue}; // Empty rows are ignored.
            let mut i = each.split(":");

            let property = i.next().unwrap();
            let value = i.next().unwrap();
            match property  {
                "name" => decoded_name = value,
                "type" => decoded_type = value,
                "size" => decoded_size = value,
                "pktn" => decoded_pktn = value,
                _ => println!("unknown meta data '{}' containing '{}'", property, value),
            };
        } // Handle packets that don't start with 'meta'

        // Has to handle incomplete packets. TODO
        {
            decoded_cont = split.next().unwrap();
            let len = decoded_cont.len() -2;
            decoded_cont = &decoded_cont[..len];
        }

        println!("name '{}' \t type '{}' \t size '{}b'", decoded_name,decoded_type,decoded_cont.len());
    }

    match decoded_type {
        "file" => {
            let path = ["examples/", decoded_name].concat();
            let mut f = File::create(path)?;
            f.write_all(decoded_cont.as_bytes())?;
        },
        "disp" => println!("{}", decoded_cont), // TODO this needs to be changed if the use of a real interface would be implemented.
        _ => println!("unknown meta type '{}', discarding package", decoded_type),

    }

    Ok(())

}

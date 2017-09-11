use std::net::TcpStream;
use std::io::prelude::*;
use std::io;
use std::fs::{File, OpenOptions};

pub fn file(mut file: File, title: &str) -> Result<Vec<String>, io::Error> {
    let mut content: String = String::new();
    file.read_to_string(&mut content);
    let res = generic("file", &mut content, title)?;
    Ok(res)
}

pub fn generic(encoded_type: &str, mut content: &mut String, title: &str) -> Result<Vec<String>, io::Error> {
    let stdbuf = super::STDBUF-154;

    let size = content.len();
    let mut split_at = 0;

    let mut packages = Vec::new();

    let mut pktnr = 1; // Meta data depicting which packet this is. // TODO should this start at 0?
    while split_at != content.len() {
        pktnr += 1;

        match (super::STDBUF + split_at) > content.len() {
            true => split_at = content.len(),
            false => split_at += stdbuf,
        };

        // The lower end of the string slice.
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
                        name:", title, ";\
                        pktnr:", pktnr.to_string().as_str(), ";\
                        size:", content[min..split_at].as_ref(),
            ";}\
             }\
                ", super::format_tag("cont", content[min..split_at].as_ref()).as_str() , "\
             }"
        ].concat());
    };
    Ok(packages)
}
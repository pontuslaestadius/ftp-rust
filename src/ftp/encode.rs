use std::net::TcpStream;
use std::io::prelude::*;
use std::io;
use std::fs::{File, OpenOptions};

use super::metadata;
use super::format_tag;

pub fn file(mut file: File, title: &str) -> Result<Vec<String>, io::Error> {
    let mut content: String = String::new();
    file.read_to_string(&mut content);
    let res = generic("file", &mut content, title)?;
    Ok(res)
}

pub fn generic(encoded_type: &str, mut content: &mut String, title: &str) -> Result<Vec<String>, io::Error> {
    let stdbuf = super::STDBUF-154; // Don't use -154, use length of metadata. TODO

    let size = content.len();
    let mut split_at = 0;

    let mut packages = Vec::new();

    let mut pktnr = 1; // Meta data depicting which packet this is.
    while split_at != content.len() {

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

        let metadata: String = ["type:", encoded_type, ";\
                        name:", title, ";\
                        pktn:", pktnr.to_string().as_str(), ";\
                        size:", content[min..split_at].len().to_string().as_str(),
            ";"].concat();

        // TODO make this process more modular.
        packages.push(["{",
            super::format_tag("meta", metadata.as_str()).as_str(),
            super::format_tag("cont", content[min..split_at].as_ref()).as_str() , "\
             }"].concat());
        pktnr += 1;
    };
    Ok(packages)
}

pub fn from_meta_data(metadata: metadata::Metadata, mut content: &mut String) -> Result<Vec<String>, io::Error> {
    let md = format_tag("meta", metadata.format().as_str());

    // -6 comes from the bytes 'cont{}'. By removing the metadata length and the 6 bytes the length
    // will get full use of the STDBUF set.
    let stdbuf = super::STDBUF -md.len() -6;

    let size = content.len();
    let mut split_at = 0;

    let mut packages = Vec::new();

    let mut pktnr = 1; // Meta data depicting which packet this is.
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
        packages.push(["}",
            md.as_str(),
            super::format_tag("cont", content[min..split_at].as_ref()).as_str() , "\
             }"].concat());
    }
    Ok(packages)
}
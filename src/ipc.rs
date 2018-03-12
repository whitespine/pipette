use std::os::unix::net::UnixStream;

static TERMINATOR : &[u8] = b"!#TERM_HEADER#!";


use serde_json;
use std::io::{Read, Write};
use std::string::String;

#[derive(Serialize, Deserialize)]
pub enum ClientRequestHeader {
    ReadFromSpout(String),  
    WriteToSink(String),
    CreatePipePair(String),
    DestroyPipePair(String),
}

#[derive(Serialize, Deserialize)]
pub enum DaemonResponse {
    Confirm,
    Deny(String),
}

pub fn send_request_header(request : ClientRequestHeader, stream : &mut UnixStream) {
    // Convert to serialized message
    let serialized = serde_json::to_string(&request).expect("Failed to serialize request header");
    
    // Send request + terminator
    stream.write_all(serialized.as_bytes()).expect("Failed to send request header");
    stream.write_all(TERMINATOR);
}

pub fn read_stream_until(stream : &mut UnixStream, term : &[u8]) -> Result<String, ()> {
    // Read until header. Messy, but oh well
    let byte_buff = &mut ['\0' as u8];
    let mut read_so_far : Vec<u8> = Vec::new();
    let mut last_n : Vec<u8> = Vec::new();

    // Loop until found terminator
    while last_n != term {
        // Get one character
        stream.read_exact(byte_buff);
        let next_byte = &mut byte_buff[0];
        read_so_far.push(*next_byte);
        last_n.push(*next_byte);
        
        // Shorten last_n if necessary
        if last_n.len() > term.len() {
            last_n.remove(0);
        }
    }
    
    // Convert into string
    let amt_to_keep : usize = read_so_far.len() - term.len();
    read_so_far.truncate(amt_to_keep);
    let read_so_far = String::from_utf8(read_so_far).expect("Failed to parse header into String");
    Ok(read_so_far)
}

pub fn read_response_header(stream : &mut UnixStream) -> DaemonResponse {
    // Try to parse header as, well, the header
    DaemonResponse::Confirm
}

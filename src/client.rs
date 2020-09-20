use std::os::unix::net::UnixStream;
use std::io::prelude::*;

fn main() {
    let mut socket = match UnixStream::connect("/tmp/pfych-rust.sock") {
        Ok(socket) => socket,
        Err(e) => {
            println!("Could not connect {:?}", e);
            return
        }
    };

    match socket.write_all(b"Hello World!") {
        Ok(_) => {},
        Err(_) => panic!("Could not send message")
    }

    println!("{:?}", socket)
}

mod resp;
mod sync;
#[cfg(test)]
mod resp_test;

use std::{io::{Read, Write}, net::TcpListener};

use crate::{sync::{read_command, respond_to_command}};


fn main(){
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    let mut connections: u32 = 0;

    loop {
        let ( mut stream, socket_address ) = listener.accept().unwrap();
        
        connections += 1;

        println!("New Connection from address{:?},  total connections: {}", socket_address.ip(), connections);

        loop {
            let mut buffer = [0; 1024];

            match stream.read(&mut buffer){
                Ok(0) => {
                    println!("Connection Closed with ip {:?}", socket_address.ip());
                    connections -= 1;
                    break;
                },
                Ok(n) => {
                    println!("Packets Received: {}", n);
                    println!("Sending it back");
                    let command = read_command(&buffer[..n]).unwrap();
                    respond_to_command(&mut stream, command);
                    
                },
                Err(err) => println!("Error {:?}", err.to_string())
            }
        }

    }
}
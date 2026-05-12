mod resp;
mod sync;

use mio::{Events, Interest, Poll, Token};
use mio::net::TcpListener;
use std::io::Read;

use crate::sync::respond_to_command;

const SERVER: Token = Token(0);

fn main() -> std::io::Result<()> {
    // 1. Create the poll instance (wraps kqueue fd)
    let mut poll = Poll::new()?;

    // 2. Event buffer (same as `struct kevent events[128]`)
    let mut events = Events::with_capacity(128);

    // 3. Create and register TCP listener
    let addr = "127.0.0.1:8080".parse().unwrap();
    let mut server = TcpListener::bind(addr)?;

    // Register server fd with kqueue for READ readiness
    poll.registry().register(&mut server, SERVER, Interest::READABLE)?;

    // Track client connections
    let mut clients: Vec<Option<mio::net::TcpStream>> = Vec::new();

    // 4. Event loop
    loop {
        // This calls kevent() under the hood — blocks until something is ready
        poll.poll(&mut events, None)?;

        for event in events.iter() {
            match event.token() {
                SERVER => {
                    // Server fd is readable → accept new connection
                    let (mut client, addr) = server.accept()?;
                    println!("New connection from {}", addr);

                    let token = Token(clients.len() + 1);
                    poll.registry().register(&mut client, token, Interest::READABLE | Interest::WRITABLE)?;
                    clients.push(Some(client));
                }
                token => {
                    // A client fd is readable
                    let idx = token.0 - 1;
                    if let Some(ref mut client) = clients[idx] {
                        let mut buf = [0u8; 1024];
                        match client.read(&mut buf) {
                            Ok(0) => {
                                println!("Client disconnected");
                                clients[idx] = None;
                            }
                            Ok(n) => {
                                let command = sync::read_command(&buf[..n]).unwrap();
                                respond_to_command(client, command);
                            }
                            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                                // Not actually ready yet, try again later
                            }
                            Err(e) => {
                                println!("Error: {}", e);
                                clients[idx] = None;
                            }
                        }
                    }
                }
            }
        }
    }
}
use std::io::{ Read, Write };
use std::net::{TcpListener, TcpStream};

struct Transfer {
    fromAddress: String,
    toAddress: String,
    message: String,
}

impl Transfer {
    fn transfer(&self) -> Result {
        match TcpSteam::connect(self.toAddress) {
            Ok(mut conn) =>{
                conn.write_all(self.message.as_bytes()).is_ok();
            }

            Err(_)=>{
                false
            }
        }
    }

}
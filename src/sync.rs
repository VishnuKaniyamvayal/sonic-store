use std::{io::Write, net::TcpStream};

use crate::resp::{RespType, decode_arguments};

pub struct SonicCommand {
    pub name: String,
    pub args: Vec<String>,
}

pub fn encode_resp(value: &RespType) -> Vec<u8> {
    match value {
        RespType::SimpleString { data, .. } => {
            format!("+{}\r\n", data).into_bytes()
        },
        RespType::Error { message, .. } => {
            format!("-{}\r\n", message).into_bytes()
        },
        RespType::Integer { data, .. } => {
            format!(":{}\r\n", data).into_bytes()
        },
        RespType::BulkString { data, .. } => {
            let mut result = format!("${}\r\n", data.len()).into_bytes();
            result.extend_from_slice(data);
            result.extend_from_slice(b"\r\n");
            result
        },
        RespType::Array { data, .. } => {
            let mut result = format!("*{}\r\n", data.len()).into_bytes();
            for item in data {
                result.extend(encode_resp(item));
            }
            result
        },
    }
}

pub fn respond_to_command(stream: &mut TcpStream ,command: SonicCommand) {

    // evaluate the command and write the response to the stream
    
    match command.name.as_str() {
        "PING" => {
            if command.args.len() == 0 {
                let response = encode_resp(&RespType::SimpleString { data: "PONG".to_string(), delta: 0 });
                stream.write_all(&response).unwrap();
            }
            else if command.args.len() >= 2 {
                respond_error(stream, "ERR wrong number of arguments for 'PING' command");
            }
            else {
                let response = encode_resp(&RespType::SimpleString { data: command.args[0].clone(), delta: 0 });
                stream.write_all(&response).unwrap();
            }
        },
        _ => respond_error(stream, "ERR unknown command"),
    }
}

pub fn respond_error(stream: &mut TcpStream, message: &str) {
    let response = encode_resp(&RespType::Error { message: message.to_string(), delta: 0 });
    stream.write_all(&response).unwrap();
}

pub fn read_command(raw_data: &[u8]) -> Result<SonicCommand, String> {
    println!("Raw data: {:?}", decode_arguments(raw_data));
    match decode_arguments(raw_data) {
        Ok(tokens) => {
            if tokens.len() == 0 {
                return Err("No command found".to_string());
            }
                println!("{:?}", tokens);
                let name = match &tokens[0] {
                    RespType::SimpleString { data, .. } => data.clone(),
                    RespType::BulkString { data, .. } => {
                        let mut result = String::new();
                        for byte in data {
                            result.push(*byte as char);
                        }
                        result
                    },
                    RespType::Integer { data, .. } => data.to_string(),
                    RespType::Array { data, .. } => {
                        return Err("Command name cannot be an array".to_string());
                    },
                     _ => return Err("Invalid command name type".to_string()),
                };

                let args = tokens[1..].iter().map(|token| {
                    match token {
                        RespType::SimpleString { data, .. } => Ok(data.clone()),
                        RespType::BulkString { data, .. } => {
                            let mut result = String::new();
                            for byte in data {
                                result.push(*byte as char);
                            }
                            Ok(result)
                        },
                        RespType::Integer { data, .. } => Ok(data.to_string()),
                        _ => Err("Unsupported argument type".to_string()),
                    }
                }).collect::<Result<Vec<String>, String>>()?;
                
                return  Ok( SonicCommand{ name, args });
        },
        Err(err) => Err(format!("Failed to decode command: {:?}", err))
        //  Err(format!("Failed to decode command: {:?}", "Not implemented"))
    }
}
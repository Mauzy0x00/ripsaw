use std::net::TcpStream;
use std::net::{IpAddr, SocketAddr};

use crate::library;

pub fn ssh_socket(addr: String, port: u16, user: String, password: String, config: library::Config) {
    // Connect to the local SSH server
    let a: IpAddr = addr.parse().expect("parse failed");
    let socket = SocketAddr::from((a, port)); 
    let tcp = TcpStream::connect(socket).unwrap();
    let mut session = ssh2::Session::new().unwrap();
    session.set_tcp_stream(tcp);
    session.handshake().unwrap();

    session.userauth_password(&user, &password).unwrap();
    assert!(session.authenticated());
    println!("logged in");
}


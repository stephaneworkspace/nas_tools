use ssh2::Session;
// use std::env;
use std::io::prelude::*;
use std::net::TcpStream;

pub fn halt_nas(login: &str, password: &str) {
    // Connect to the local SSH server
    let tcp = TcpStream::connect("192.168.1.123:22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    sess.userauth_password(login, password).unwrap();

    sess.set_banner("-oHostKeyAlgorithms=+ssh-dss").unwrap();

    let mut channel = sess.channel_session().unwrap();
    channel.exec("halt -n").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    channel.wait_close().unwrap();
    println!("{}", channel.exit_status().unwrap());
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

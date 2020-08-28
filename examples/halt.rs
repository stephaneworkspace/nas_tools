use ssh2::Session;
use std::env;
use std::net::TcpStream;

fn main() {
    // Connect to the local SSH server
    let tcp = TcpStream::connect("192.168.1.123:22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    let args: Vec<String> = env::args().collect();

    let login = &args[1];
    let password = &args[2];
    sess.userauth_password(login, password).unwrap();
}

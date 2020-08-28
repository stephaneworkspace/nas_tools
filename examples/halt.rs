use nas_tools::halt;
use std::env;

fn main() {
    // Connect to the local SSH server
    let args: Vec<String> = env::args().collect();
    let login = &args[1];
    let password = &args[2];
    halt(login, password);
}

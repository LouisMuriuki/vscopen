use std::env;
use std::process::{ exit, Command };
fn main() {
    let args: Vec<String> = env::args().collect();
    let help = env::args().any(|arg| (arg == "--help" || arg == "-h"));

    if help {
        println!("From source");
    }
    let status = Command::new("code")
        .args(&args[1..])
        .spawn();

    match status {
        Ok(_) => (),
        Err(e) => {
            println!("Error: {}", e);
            exit(0)
        }
    }
}

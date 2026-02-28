use std::process;

fn main() {
    if let Err(e) = bonsai::run() {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

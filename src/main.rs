pub mod cli;
pub mod config;
pub mod errors;
pub mod keys;

fn main() {
    if let Err(e) = cli::run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

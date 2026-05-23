pub mod errors;
pub mod keys;
pub mod cli;
pub mod config;

fn main() {
    if let Err(e) = cli::run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

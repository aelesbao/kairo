fn main() {
    if let Err(e) = kairo_cli::run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

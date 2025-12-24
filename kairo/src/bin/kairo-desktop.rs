fn main() {
    if let Err(e) = kairo_desktop::run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

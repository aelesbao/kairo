fn main() {
    match kairo_cli::run() {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Error: {}", e);
            let code = match e {
                kairo_cli::Error::OpenUrl(_, status) => status.code(),
                _ => None,
            };
            std::process::exit(code.unwrap_or(1));
        }
    }
}

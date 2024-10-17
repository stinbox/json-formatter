use std::{env, fs};

fn main() {
    let mut args = env::args();

    args.next();

    let filename = match args.next() {
        Some(filename) => filename,
        None => {
            eprintln!("No filename provided");
            std::process::exit(1);
        }
    };

    let content = match fs::read_to_string(&filename) {
        Ok(content) => content,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    eprintln!("No such file or directory: '{}'", filename);
                }
                std::io::ErrorKind::PermissionDenied => {
                    eprintln!("Permission denied: '{}'", filename);
                }
                _ => {
                    eprintln!("Error reading file '{}': {}", filename, error);
                }
            }
            std::process::exit(1);
        }
    };

    let formatted = json_formatter::format_json(content)
        .expect("Failed to format JSON");

    println!("{}", formatted);
}

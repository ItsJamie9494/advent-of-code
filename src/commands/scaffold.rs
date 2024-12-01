use std::fs::{File, OpenOptions};
use std::io::Write;

const TEMPLATE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/TEMPLATE.txt"));

fn create_file(path: &str) -> std::io::Result<File> {
    let mut file = OpenOptions::new();
    file.create_new(true);
    file.truncate(true).write(true).open(path)
}

pub fn scaffold(day: String) {
    let module_path = format!("{}/src/bin/{day}.rs", env!("CARGO_MANIFEST_DIR"));

    let mut file = match create_file(&module_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create module file: {e}");
            std::process::exit(1);
        }
    };

    match file.write_all(
        TEMPLATE
            .replace("%DAY%", &*day).as_bytes(),
    ) {
        Ok(()) => {
            println!("Created module file \"{}\"", &module_path);
        }
        Err(e) => {
            eprintln!("Failed to write module contents: {e}");
            std::process::exit(1);
        }
    };
}
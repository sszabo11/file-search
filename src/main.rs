use colored::*;
use std::{
    fs,
    io::{self, stdin},
    thread,
    time::Instant,
};

fn main() {
    println!("{}", "Search blazingly fast!".bold().yellow());
    run()
}

fn run() {
    loop {
        let mut target: String = String::new();

        println!("{}", "\nFile or directory to search:".cyan());

        match stdin().read_line(&mut target) {
            Ok(_) => {
                println!("Searching for {:?}...", target.trim());
                let target = target.trim();
                let start = Instant::now();
                let result = find("C:/", &target);

                match result {
                    Ok(path) => {
                        println!("{} {}", "Found target at:".green(), path.bright_cyan());
                        let duration = start.elapsed();
                        println!(
                            "Found in {} {}",
                            duration.as_millis().to_string().green(),
                            "milliseconds"
                        );
                    }
                    Err(err) => {
                        eprintln!(
                            "{}: {}",
                            "Failed to read target".bright_red(),
                            err.to_string().bright_red()
                        )
                    }
                }
            }
            Err(err) => {
                eprintln!("Failed to read input: {}", err)
            }
        }
    }
}

fn find(path: &str, target: &str) -> Result<String, io::Error> {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            match entry {
                Ok(file) => {
                    let path = file.path();
                    if let Some(name) = path.file_name() {
                        if name == target {
                            return Ok(path.to_str().unwrap().to_string());
                        } else if path.is_dir() {
                            match find(path.to_str().unwrap(), target) {
                                Ok(found_path) => return Ok(found_path),
                                Err(_) => continue,
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading entry: {:?}", e);
                }
            }
        }
    } else {
        eprintln!("Failed to read direcoty: {}", path)
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "File not found"))
}

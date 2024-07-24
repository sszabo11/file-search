use colored::*;
use dialoguer::Select;
use rayon::prelude::*;
use std::{
    fs,
    io::{self, stdin},
    time::Instant,
};

fn main() {
    println!("{}", "Search blazingly fast!".bold().yellow());
    // run()

    let items = vec!["Return multiple files", "Return single file"];
    let selection = Select::new()
        .with_prompt("What do you choose?")
        .items(&items)
        .interact()
        .unwrap();

    println!("You chose: {}", items[selection]);
}

fn run() {
    loop {
        let mut target: String = String::new();
        let mut multiple: bool = false;

        println!("{}", "\nFile or directory to search:".cyan());

        match stdin().read_line(&mut target) {
            Ok(_) => {
                println!("Searching for {:?}...", target.trim());
                let target = target.trim();
                let start = Instant::now();
                let result = find("C:\\", &target, multiple);

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

fn find(path: &str, target: &str, multiple: bool) -> Result<String, io::Error> {
    let entries = fs::read_dir(path)?;

    let results: Vec<_> = entries
        .filter_map(|entry| entry.ok())
        .par_bridge()
        .filter_map(|entry| {
            let path = entry.path();
            if let Some(name) = path.file_name() {
                if name == target {
                    return Some(Ok::<String, String>(path.to_str().unwrap().to_string()));
                } else if path.is_dir() {
                    return match find(path.to_str().unwrap(), target, multiple) {
                        Ok(found_path) => Some(Ok(found_path)),
                        Err(_) => None,
                    };
                }
            }
            None
        })
        .collect();

    for result in results {
        if let Ok(found_path) = result {
            return Ok(found_path);
        }
    }

    Err(io::Error::new(io::ErrorKind::NotFound, "File not found"))

    // println!("{:?}", fs::read_dir(path));
    // if let Ok(entries) = fs::read_dir(path) {
    //     for entry in entries {
    //         match entry {
    //             Ok(file) => {
    //                 let path = file.path();
    //                 if let Some(name) = path.file_name() {
    //                     if name == target {
    //                         return Ok(path.to_str().unwrap().to_string());
    //                     } else if path.is_dir() {
    //                         match find(path.to_str().unwrap(), target) {
    //                             Ok(found_path) => return Ok(found_path),
    //                             Err(_) => continue,
    //                         }
    //                     }
    //                 }
    //             }
    //             Err(e) => {
    //                 eprintln!("Error reading entry: {:?}", e);
    //             }
    //         }
    //     }
    // } else {
    //     // eprintln!("Failed to read directory: {path}");
    // }
    // Err(io::Error::new(io::ErrorKind::NotFound, "File not found"))
}

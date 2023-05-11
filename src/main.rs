use clap::{Parser, Subcommand};
use std::{
    fs,
    io::Result,
    path::Path,
    process::{self, Command},
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    verbose: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Python3 { fname: String },
    Python2 { fname: String },
    Shell { fname: String },
    Bash { fname: String },
}

enum Filetype {
    Python3,
    Python2,
    Shell,
    Bash,
}

struct File {
    filename: String,
    filetype: Filetype,
}

impl File {
    // Associative funstions.
    fn file_python3(fname: String) -> Result<File> {
        let filename = format!("{}.py", fname);
        println!("{} - Python 3", filename);
        match Path::new(&filename).try_exists() {
            Ok(value) => {
                if !value {
                    Ok(File {
                        filename,
                        filetype: Filetype::Python3,
                    })
                } else {
                    println!("The file exists. Terminating!");
                    process::exit(1);
                }
            }
            Err(e) => Err(e),
        }
    }
    fn file_python2(fname: String) -> Result<File> {
        let filename = format!("{}.py", fname);
        println!("{} - Python 2", filename);
        match Path::new(&filename).try_exists() {
            Ok(value) => {
                if !value {
                    Ok(File {
                        filename,
                        filetype: Filetype::Python2,
                    })
                } else {
                    println!("The file exists. Terminating!");
                    process::exit(1);
                }
            }
            Err(e) => Err(e),
        }
    }
    fn file_shell(fname: String) -> Result<File> {
        let filename = format!("{}.sh", fname);
        println!("{} - Shell", filename);
        match Path::new(&filename).try_exists() {
            Ok(value) => {
                if !value {
                    Ok(File {
                        filename,
                        filetype: Filetype::Shell,
                    })
                } else {
                    println!("The file exists. Terminating!");
                    process::exit(1);
                }
            }
            Err(e) => Err(e),
        }
    }
    fn file_bash(fname: String) -> Result<File> {
        let filename = format!("{}.bash", fname);
        println!("{} - Bash", filename);
        match Path::new(&filename).try_exists() {
            Ok(value) => {
                if !value {
                    Ok(File {
                        filename,
                        filetype: Filetype::Bash,
                    })
                } else {
                    println!("The file exists. Terminating!");
                    process::exit(1);
                }
            }
            Err(e) => Err(e),
        }
    }
    // Methods.
    fn create_file(&self) -> Result<()> {
        let input_string = match self.filetype {
            Filetype::Python3 => String::from("#!/usr/bin/python3"),
            Filetype::Python2 => String::from("#!/usr/bin/python2"),
            Filetype::Shell => String::from("#!/bin/sh"),
            Filetype::Bash => String::from("#!/bin/bash"),
        };
        fs::write(self.filename.to_owned(), input_string)?;
        if cfg!(unix) {
            Command::new("chmod")
                .args(["751", &self.filename])
                .output()?;
        }
        Ok(())
    }
}

fn main() {
    let cli = Cli::parse();
    if cli.verbose {
        println!("Validating the file");
    }
    let file = match cli.command {
        Commands::Python3 { fname } => File::file_python3(fname),
        Commands::Python2 { fname } => File::file_python2(fname),
        Commands::Shell { fname } => File::file_shell(fname),
        Commands::Bash { fname } => File::file_bash(fname),
    };
    if file.is_ok() && cli.verbose {
        println!("File Validated. Creating the file");
    }
    match file {
        Ok(value) => value.create_file().expect("File was not created"),
        Err(e) => {
            println!("Error!, {}", e);
            process::exit(1);
        }
    }
    if cli.verbose && cfg!(unix) {
        println!("File created. Permissions granted.");
    } else if cli.verbose {
        println!("File created.");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn os_test() {
        assert!(cfg!(unix));
    }
}

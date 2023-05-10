use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
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

struct File {
    filename: String,
}

impl File {
    fn file_python3(fname: String) -> File {
        let filename = format!("{}.py", fname);
        println!("{} - Python 3", filename);
        File { filename }
    }
    fn file_python2(fname: String) -> File {
        let filename = format!("{}.py", fname);
        println!("{} - Python 2", filename);
        File { filename }
    }
    fn file_shell(fname: String) -> File {
        let filename = format!("{}.sh", fname);
        println!("{} - Shell", filename);
        File { filename }
    }
    fn file_bash(fname: String) -> File {
        let filename = format!("{}.bash", fname);
        println!("{} - Bash", filename);
        File { filename }
    }
}

fn main() {
    let cli = Cli::parse();
    let filename: File = match cli.command {
        Commands::Python3 { fname } => File::file_python3(fname),
        Commands::Python2 { fname } => File::file_python2(fname),
        Commands::Shell { fname } => File::file_shell(fname),
        Commands::Bash { fname } => File::file_bash(fname),
    };
}

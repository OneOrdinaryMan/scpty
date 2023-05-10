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
fn main() {
    let cli = Cli::parse();
    let _filename: String = match &cli.command {
        Commands::Python3 { fname } => {
            let filename = format!("{}.py", fname);
            println!("{} - Python 3", filename);
            filename
        }
        Commands::Python2 { fname } => {
            let filename = format!("{}.py", fname);
            println!("{} - Python 2", filename);
            filename
        }
        Commands::Shell { fname } => {
            let filename = format!("{}.sh", fname);
            println!("{} - Shell", filename);
            filename
        }
        Commands::Bash { fname } => {
            let filename = format!("{}.bash", fname);
            println!("{} - Bash", filename);
            filename
        }
    };
}

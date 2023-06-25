use clap::Parser;
use scpty::{cli, repo_creation::RepoCreator};
use std::{io::Result, path::PathBuf, process};

fn main() -> Result<()> {
    let asset_dir = "/usr/share/scpty";
    let asset = PathBuf::from(asset_dir);

    match asset.try_exists() {
        Ok(value) => {
            if !value {
                eprintln!("The asset dir doesn't exist. Exiting");
                process::exit(1);
            }
            Ok(())
        }
        Err(err) => Err(err),
    }?;

    let cli = cli::Cli::parse();
    let mut repo_creator = RepoCreator::new(cli.lang(), asset, cli.name());
    repo_creator.verbose_toggle(cli.verbose());
    repo_creator.create_repo()?;

    Ok(())
}

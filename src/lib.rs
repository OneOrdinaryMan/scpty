pub mod repo_creation {

    use std::{
        env, fs,
        io::Result,
        path::PathBuf,
        process::{self, Command},
    };

    pub enum Lang {
        Shell,
        Bash,
        Python,
    }

    pub struct RepoCreator {
        repo_lang: Lang,
        asset: PathBuf,
        name: String,
        verbose: bool,
    }

    impl RepoCreator {
        pub fn new(repo_lang: Lang, asset: PathBuf, name: String) -> RepoCreator {
            RepoCreator {
                repo_lang,
                asset,
                name,
                verbose: false,
            }
        }
    }

    impl RepoCreator {
        pub fn verbose_toggle(&mut self, toggle: bool) {
            self.verbose = toggle;
        }

        pub fn create_repo(&self) -> Result<()> {
            match self.repo_lang {
                Lang::Shell => self.shell_repo(),
                Lang::Bash => self.bash_repo(),
                Lang::Python => self.python_repo(),
            }?;

            Ok(())
        }

        fn shell_repo(&self) -> Result<()> {
            if self.verbose {
                println!("Making the directory");
            }

            let project_dir: PathBuf = PathBuf::from(self.name.as_str());
            fs::create_dir(project_dir.as_path())?;
            let shell_asset: PathBuf = self.asset.as_path().join("shell.tar.gz");

            if self.verbose {
                println!("Done!\nChecking the asset file.");
            }

            if !shell_asset.try_exists()? {
                eprintln!("The asset file does'nt exists.");
                process::exit(1);
            }

            if self.verbose {
                println!("Asset exists!\nExtracting files in the project directory");
            }

            let untar_output = Command::new("tar")
                .arg("-xvf")
                .arg(shell_asset)
                .arg("-C")
                .arg(project_dir.as_path())
                .arg(".")
                .output()?;
            process_output(untar_output, self.verbose);

            if self.verbose {
                println!("Extraction Complete!");
                println!("Initialising git!");
            }

            let git_init_output = Command::new("git")
                .args(["init", "-b", "main"])
                .arg(project_dir.as_path())
                .output()?;
            process_output(git_init_output, self.verbose);

            if self.verbose {
                println!("Staging the git");
            }

            env::set_current_dir(project_dir.as_path())?;
            let git_add_output = Command::new("git").args(["add", "."]).output()?;
            process_output(git_add_output, self.verbose);

            if self.verbose {
                println!("Staged the git");
                println!("Making the initial commit.");
            }

            let git_commit_output = Command::new("git")
                .args(["commit", "-m", "Initial commit"])
                .output()?;
            process_output(git_commit_output, self.verbose);

            if self.verbose {
                println!("Initial commit created.");
            }

            println!("Project directory built for shell.");

            Ok(())
        }

        fn bash_repo(&self) -> Result<()> {
            if self.verbose {
                println!("Making the directory");
            }

            let project_dir: PathBuf = PathBuf::from(self.name.as_str());
            fs::create_dir(project_dir.as_path())?;
            let bash_asset: PathBuf = self.asset.as_path().join("bash.tar.gz");

            if self.verbose {
                println!("Done!\nChecking the asset file.");
            }

            if !bash_asset.try_exists()? {
                eprintln!("The asset file does'nt exists.");
                process::exit(1);
            }

            if self.verbose {
                println!("Asset exists!\nExtracting files in the project directory");
            }

            let untar_output = Command::new("tar")
                .arg("-xvf")
                .arg(bash_asset)
                .arg("-C")
                .arg(project_dir.as_path())
                .arg(".")
                .output()?;
            process_output(untar_output, self.verbose);

            if self.verbose {
                println!("Extraction Complete!");
                println!("Initialising git!");
            }

            let git_init_output = Command::new("git")
                .args(["init", "-b", "main"])
                .arg(project_dir.as_path())
                .output()?;
            process_output(git_init_output, self.verbose);

            if self.verbose {
                println!("Staging the git");
            }

            env::set_current_dir(project_dir.as_path())?;
            let git_add_output = Command::new("git").args(["add", "."]).output()?;
            process_output(git_add_output, self.verbose);

            if self.verbose {
                println!("Staged the git");
                println!("Making the initial commit.");
            }

            let git_commit_output = Command::new("git")
                .args(["commit", "-m", "Initial commit"])
                .output()?;
            process_output(git_commit_output, self.verbose);

            if self.verbose {
                println!("Initial commit created.");
            }

            println!("Project directory built for bash.");

            Ok(())
        }

        fn python_repo(&self) -> Result<()> {
            if self.verbose {
                println!("Making the directory");
            }

            let project_dir: PathBuf = PathBuf::from(self.name.as_str());
            fs::create_dir(project_dir.as_path())?;
            let python_asset: PathBuf = self.asset.as_path().join("python.tar.gz");

            if self.verbose {
                println!("Done!\nChecking the asset file.");
            }

            if !python_asset.try_exists()? {
                eprintln!("The asset file does'nt exists.");
                process::exit(1);
            }

            if self.verbose {
                println!("Asset exists!\nExtracting files in the project directory");
            }

            let untar_output = Command::new("tar")
                .arg("-xvf")
                .arg(python_asset)
                .arg("-C")
                .arg(project_dir.as_path())
                .arg(".")
                .output()?;
            process_output(untar_output, self.verbose);

            if self.verbose {
                println!("Extraction Complete!");
                println!("Initialising git!");
            }

            let git_init_output = Command::new("git")
                .args(["init", "-b", "main"])
                .arg(project_dir.as_path())
                .output()?;
            process_output(git_init_output, self.verbose);

            if self.verbose {
                println!("Staging the git");
            }

            env::set_current_dir(project_dir.as_path())?;
            let git_add_output = Command::new("git").args(["add", "."]).output()?;
            process_output(git_add_output, self.verbose);

            if self.verbose {
                println!("Staged the git");
                println!("Making the initial commit.");
            }

            let git_commit_output = Command::new("git")
                .args(["commit", "-m", "Initial commit"])
                .output()?;
            process_output(git_commit_output, self.verbose);

            if self.verbose {
                println!("Initial commit created.");
            }

            println!("Project directory built for python.");

            Ok(())
        }
    }
    fn process_output(output: process::Output, verbose: bool) {
        let stdout: String = match String::from_utf8(output.stdout.to_vec()) {
            Ok(value) => value,
            Err(_) => String::new(),
        };
        let stderr: String = match String::from_utf8(output.stderr.to_vec()) {
            Ok(value) => value,
            Err(_) => String::new(),
        };
        if !output.status.success() {
            eprintln!("{}\nExtracting failed.", stderr);
            process::exit(1);
        }

        if verbose && !stdout.is_empty() {
            println!("{}", stdout);
        }
    }
}

pub mod cli {
    use super::repo_creation::Lang;
    use clap::Parser;
    use std::process;

    #[derive(Parser)]
    #[command(author, version, about, long_about = None)]
    pub struct Cli {
        #[arg(short, long)]
        verbose: bool,

        #[arg(short, long)]
        lang: String,

        name: String,
    }

    impl Cli {
        pub fn name(&self) -> String {
            self.name.to_owned()
        }

        pub fn lang(&self) -> Lang {
            match self.lang.as_str() {
                "shell" => Lang::Shell,
                "bash" => Lang::Bash,
                "python" => Lang::Python,
                _ => {
                    eprintln!("The lang argument is incorrect.");
                    process::exit(1);
                }
            }
        }

        pub fn verbose(&self) -> bool {
            self.verbose
        }
    }

    #[cfg(test)]
    mod tests {
        use std::process::Command;
        #[test]
        fn stdout_test() {
            let output = Command::new("./test_assets/std_out").output().unwrap();
            let stdout: String = match String::from_utf8(output.stdout.to_vec()) {
                Ok(value) => value,
                Err(_) => String::new(),
            };
            assert_eq!(stdout, String::from("Hello World\n"));
        }
        #[test]
        fn stderr_test() {
            let output = Command::new("./test_assets/std_err").output().unwrap();
            let stderr: String = match String::from_utf8(output.stderr.to_vec()) {
                Ok(value) => value,
                Err(_) => String::new(),
            };
            assert_eq!(stderr, String::from("Hello World\n"));
        }
    }
}

# Scpty
Scpty is a project init tool. This is a cli tool created with the clap framework in rust.
> Project is only for unix based operating systems for now. Will add for the windows later.
# Installation
> Do not Install this project using <mark>cargo install</mark> since cargo will not install the assets needed by the project.

To install,
```sh
sudo make clean install
make remake_release
sudo make install
```

To uninstall the project,
```sh
sudo make uninstall
```
## Dependecies
- tar
- git
## Build Dependencies
- make
- cargo
- tar
## Optional Dependecies (needed to run the specific lang)
- make
- python
# Usage
Usage instructions,
```sh
scpty -l <lang> <project_name>
```
## Language options
The following languages are currently supported by scpty,

- shell
- bash
- python
# License
This project is licensed under <mark>GNU GPL v3.0 or above</mark> license. Feel free to use the project.

# Installing the cargo depemdencies
FROM rust:latest as depender
WORKDIR /usr/src/myapp
RUN ["/bin/bash", "-c", "git config --global user.email tester@test.com"]
RUN ["/bin/bash", "-c", "git config --global user.name 'tester test'"]
RUN ["/bin/bash", "-c", "cargo init"]
COPY ./Cargo.toml .
RUN ["/bin/bash", "-c", "cargo build"]
# Run the tests in the app.
FROM depender as tester
COPY . .
RUN ["/bin/bash", "-c", "cargo test"]
# Install the app
FROM tester as installer
WORKDIR /usr/src/myapp
RUN ["/bin/bash", "-c", "make clean install"]
# Help
FROM installer as help_screen
WORKDIR /
CMD ["/bin/bash", "-c", "scpty --help"]
# Shell
FROM installer as shell
WORKDIR /
CMD ["/bin/bash", "-c", "scpty -l shell hello_world_sh"]
# Bash
FROM installer as bash
WORKDIR /
CMD ["/bin/bash", "-c", "scpty -l bash hello_world_bash"]
# Python
FROM installer as python
WORKDIR /
CMD ["/bin/bash", "-c", "scpty -l python hello_world_py"]
# Uninstaller tester
FROM installer as uninstall
WORKDIR /usr/src/myapp
CMD ["/bin/bash", "-c", "make uninstall"]

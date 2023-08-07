# copy and make the files
FROM rust:latest as maker
WORKDIR /usr/src/myapp
COPY . .
RUN ["/bin/bash", "-c", "git config --global user.email 'tester@test.com'"]
RUN ["/bin/bash", "-c", "git config --global user.name 'Tester test'"]
RUN ["/bin/bash", "-c", "./configure"]
RUN ["/bin/bash", "-c", "make"]
# run make check
FROM maker as checker
CMD ["/bin/bash", "-c", "make check"]
# install the app
FROM maker as installer
RUN ["/bin/bash", "-c", "make install"]
# run scpty help
FROM installer as help
CMD ["/bin/bash", "-c", "scpty -h"]
# create a bash directory
FROM installer as bash
CMD ["/bin/bash", "-c", "scpty -l bash hello_bash"]
# create a python directory
FROM installer as python
CMD ["/bin/bash", "-c", "scpty -l python hello_python"]
# create a shell directory
FROM installer as shell
CMD ["/bin/bash", "-c", "scpty -l shell hello_shell"]

FROM gcc:latest as installer
WORKDIR /usr/src/myapp
COPY . .
RUN ["/bin/bash", "-c", "./configure"]
RUN ["/bin/bash", "-c", "make"]
RUN ["/bin/bash", "-c", "make install"]
FROM installer as runner
CMD ["/bin/bash", "-c", "hello_bash"]
FROM installer as uninstaller
CMD ["/bin/bash", "-c", "make uninstall"]

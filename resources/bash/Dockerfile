# Install the app
FROM gcc:latest as installer
COPY . /usr/src/myapp
WORKDIR /usr/src/myapp
RUN ["/bin/bash", "-c", "make remake install"]
# Run the app
FROM installer as runner
WORKDIR /
CMD ["/bin/bash", "-c","hello_world"]
# Uninstall
FROM installer as uninstall
WORKDIR /usr/src/myapp
CMD ["/bin/bash", "-c","make uninstall"]

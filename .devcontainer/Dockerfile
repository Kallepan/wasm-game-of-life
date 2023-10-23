FROM mcr.microsoft.com/devcontainers/base:debian

# Install libraries
RUN sudo apt-get update && \
    sudo apt-get install -y libglib2.0-0 libnss3 libx11-6

# Install firefox
RUN sudo apt-get install -y firefox-esr
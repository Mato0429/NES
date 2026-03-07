FROM mcr.microsoft.com/vscode/devcontainers/rust:latest

RUN apt-get update \
&& apt-get install -y x11-apps
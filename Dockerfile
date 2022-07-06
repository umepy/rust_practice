FROM rust:slim-buster

RUN apt update && apt upgrade -y
RUN apt install -y git
RUN rustup component add rls rust-analysis rust-src rustfmt
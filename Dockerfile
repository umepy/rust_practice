FROM rust:slim-buster

RUN apt update && apt upgrade -y
RUN apt install -y git
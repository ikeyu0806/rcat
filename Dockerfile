FROM rust:latest

RUN apt-get update \
  && apt-get upgrade -y \
  && apt-get install -y -q

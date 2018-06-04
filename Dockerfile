FROM rust:1.26-slim-stretch

ENV DEBIAN_FRONTEND noninteractive

RUN apt-get update -y \
      && apt-get --no-install-recommends install -y pkg-config apt-utils \
      build-essential sudo libffi-dev libssl-dev \
      && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/heroku-env

COPY . .

RUN rustup component add rustfmt-preview
RUN cargo build --release

WORKDIR /root

RUN mkdir .heroku-env/
COPY config/config.yml .heroku-env/

WORKDIR /usr/src/heroku-env

CMD ["cargo run"]

FROM rust:1.25.0-stretch

RUN apt-get update -y \
      && apt-get --no-install-recommends install -y pkg-config git-core build-essential \
      sudo libffi-dev libxml2-dev libssl-dev libcurl4-gnutls-dev curl apt-utils \
      && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/heroku-env

COPY . .

RUN cargo install
RUN cargo build --release

WORKDIR /root

RUN mkdir .heroku-env/
COPY config/config.yml .heroku-env/

WORKDIR /usr/src/heroku-env

CMD ["cargo run"]

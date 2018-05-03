FROM rust:1.25.0-stretch

RUN apt-get update -y \
      && apt-get --no-install-recommends install -y pkg-config git-core build-essential \
      sudo libffi-dev libxml2-dev libssl-dev libcurl4-gnutls-dev curl apt-utils \
      && rm -rf /var/lib/apt/lists/*

RUN curl https://cli-assets.heroku.com/install-ubuntu.sh | sh

WORKDIR /usr/src/heroku-env

COPY . .

RUN cargo install

CMD ["heroku-env"]

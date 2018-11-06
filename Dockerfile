FROM rust:1.30-stretch

ENV DEBIAN_FRONTEND noninteractive
ENV HOME /home/rusty
ENV USER rusty

RUN apt-get update -y \
      && apt-get --no-install-recommends install -y pkg-config apt-utils \
      build-essential sudo libffi-dev libssl-dev \
      && rm -rf /var/lib/apt/lists/*

# Create a local user that will be able to run commands
RUN useradd -m -s /bin/bash rusty

# Create the root directory in the home of the user
RUN mkdir -p $HOME/src/

# Creates a dummy project used to grab dependencies
WORKDIR $HOME/src/
RUN cargo new heroku-env --bin

# Switch to the newly created project directory
WORKDIR $HOME/src/heroku-env/

# Copies over *only* your manifests
COPY Cargo* $HOME/src/heroku-env/
RUN chown -R rusty:rusty $HOME/src/heroku-env

# Builds your dependencies and removes the
# fake source code from the dummy project
RUN cargo build --release
RUN rm src/*.rs

# Install Rust fmt and Clippy
RUN rustup component add rustfmt-preview
RUN rustup component add clippy-preview

# Copies only your actual source code to
# avoid invalidating the cache at all
COPY src $HOME/src/heroku-env/src

# Give the home directory the rights to the user
RUN chown -R rusty:rusty $HOME

# For some reason, the cargo cache and indexes do not seems to have the user rights
RUN chown -R rusty:rusty /usr/local/cargo

USER rusty

# Builds again, this time it'll just be
# your actual source files being built
RUN cargo build --release

# Create config and copy it to the container
RUN mkdir -p $HOME/.heroku-env/
COPY config/config.yml $HOME/.heroku-env/

CMD ["cargo run -- push"]


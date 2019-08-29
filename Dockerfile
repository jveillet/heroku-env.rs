FROM rust:1.37-stretch

RUN apt-get update -y \
      && apt-get --no-install-recommends install -y pkg-config apt-utils \
      build-essential sudo libffi-dev libssl-dev \
      && rm -rf /var/lib/apt/lists/*

ENV DEBIAN_FRONTEND noninteractive
ENV HOME /home/rusty
ENV USER rusty

# The home directory of the application
ENV APP_DIR /home/rusty/heroku-env-rs

# Create a local user that will be able to run commands
RUN useradd -m -s /bin/bash rusty

# Create the root directory in the home of the user
RUN mkdir -p $HOME/

# Creates a dummy project used to grab dependencies
WORKDIR $HOME/
RUN cargo new heroku-env-rs --bin

# Switch to the newly created project directory
WORKDIR $APP_DIR

# Copies over *only* your manifests
COPY Cargo* $APP_DIR/
COPY .env $APP_DIR/
COPY rustfmt.toml $APP_DIR/
COPY tests $APP_DIR/tests/
RUN chown -R rusty:rusty $APP_DIR

# Builds your dependencies and removes the
# fake source code from the dummy project
RUN cargo build --release
RUN rm src/*.rs

# Install Rust fmt and Clippy
RUN rustup component add rustfmt
RUN rustup component add clippy

# Copies only your actual source code to
# avoid invalidating the cache at all
COPY src $APP_DIR/src

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

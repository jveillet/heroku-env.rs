# heroku-env

A command line utility for managing config vars on Heroku via the [Platform
API](https://devcenter.heroku.com/categories/platform-api). Its primer use is to create/update config vars for one or
more apps on your Heroku pipelines.

## Disclaimer

This utility is beta, it may crash and / or behave unexpectedly, please do not use it on production
environments.

## Getting Started

### Prerequisites

You will need a version of the Rust Programming language (>= 1.25.0), it should come with Cargo, the Rust packet manager.
See the [Rust documentation](https://doc.rust-lang.org/cargo/getting-started/installation.html) for more details.

### Installing

**Note**: It only have been tested on Linux (Debian based distro). Feel free to test it on another OS
and share the outcome.

```bash
$ git clone git@gitlab.com:jveillet/heroku-env.git
$ cd heroku-env
$ cargo install
```

### Compile from source

**Note**: Same disclaimer has for the Installing part.
Another way is to build using Docker (Dockerfile and docker-compose available).

#### Compile locally

```bash
$ git clone git@gitlab.com:jveillet/heroku-env.git
$ cd heroku-env
$ cargo build --release
$ cargo run -- push
```

#### Compile with Docker

```bash
$ git clone git@gitlab.com:jveillet/heroku-env.git
$ cd heroku-env
$ docker-compose build
$ docker-compose run --rm app cargo build --release
$ docker-compose run --rm app cargo run -- push
# Or
$ docker-compose up
```

#### Rust formatting

As a best practice, this project uses Rust fmt to format the code, and comply with the [Rust
Styleguide](https://github.com/rust-lang-nursery/fmt-rfcs).

To run Rust fmt, either install it on your system with rustup:

```bash
$ cd heroku-env
$ rustup component add rustfmt-preview
$ cargo fmt
```

Or use the Docker image built for this project.

```bash
$ cd heroku-env
$ docker-compose run --rm app cargo fmt
```

## Configuration

### Heroku Personal OAuth Token

In order to use the Heroku Platform API, you must obtain a Personal OAuth Token.

If you have the Heroku CLI installed, you can extract it from the `~/.netrc` file on your system, or by launching the
command `heroku auth:token` in a terminal.

Copy the result and add it in the project in a .env file.
See this page about [dotenv](https://github.com/purpliminal/rust-dotenv) files for more informations.

```
$ cat ~/.netrc
machine api.heroku.com
  login me@example.com
  password my_api_token
```

```bash
$ cd heroku-env
$ touch .env
$ echo "HK_API_TOKEN=my_api_token" >> .env
```

Or you can export it as an environment variable into in your `~/.bashrc` or `~/.zshrc`.

```bash
$ export HK_API_TOKEN="my_api_token"
```

### Preferences

The utility needs a preferences file in order to update the config vars on Heroku, with informations about the apps and
the settings.

This file must be a YAML file, by default the tool is looking for a `config.yml` in the `~/.heroku-env/` directory.
For further informations about the configuration file location, see the [Usage](#Usage) section.

The file must be formatted like the example above:

```yaml
version: "1"
apps:
  - name: "my_app"
    settings:
      MY_TEST_VAR: "VALUE 1"
      MY_TEST_VAR_2: "VALUE 2"
  - name: "my_app_2"
    settings:
      MY_TEST_VAR: "VALUE 1"
      MY_TEST_VAR_2: "VALUE 2"
```

* version: Version of the configuration file, must be set to "1".
* apps: List of Heroku apps you want to update.
* name: name of the heroku app.
* settings: List of config vars you want to update/create for this specific app, the format is base on a `KEY: "VALUE"` pair.

## Tests

Running tests:

```bash
$ cargo test
```

## Usage

```bash
heroku-env 0.0.6
Jérémie Veillet <jeremie.veillet@gmail.com>
CLI to Update or create environment variables on Heroku written in Rust.

USAGE:
    heroku-env [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    push    Push local config vars to heroku
```

```bash
heroku-env-push
Push local config vars to heroku

USAGE:
    heroku-env push [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <FILE>    Sets a user defined config file in YAML format
```

## Contributing

You want to contribute to the Project? Yeah!! :v: 🎉  Contributors are always welcome! :thumbsup:

**Note**: One of the best ways to help right now is to use the utility and report issues!

This project started as a side project to learn Rust, so there is a lot of areas to improve.

### Bugs

If you find bugs, first go to the [issues page](https://gitlab.com/jveillet/heroku-env/issues) and search if a related issue isn't listed there.

Create a new issue and insert any informations that can help to reproduce the observed behavior:
* Command context
* Stack trace
* Expected bahevior
* Current behavior
* OS / environment

Consider adding the ~bug label on your ticket.

### Feature requests

Create a new issue on the [issues page](https://gitlab.com/jveillet/heroku-env/issues) and add a clear description of what the new feature should look like.

Consider adding the ~"feature request" label on your ticket.

### Pull Requests

1. Fork heroku-env
2. Clone your fork git clone `https://gitlab.com/$YOUR_USERNAME/heroku-env && cd heroku-env`
3. Create new branch `git checkout -b new-branch`
4. Make your changes, and commit `git commit -am "your message"`

## Licence.

heroku-env is a free software: you can redistribute it and/or modify it under the terms of the [GNU GPL v3](LICENCE).

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see http://www.gnu.org/licenses/.


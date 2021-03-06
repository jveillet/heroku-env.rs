# heroku-env

A command line utility for managing config vars on Heroku via the [Platform
API](https://devcenter.heroku.com/categories/platform-api). Its primer use is to create/update config vars for one or
more apps on your Heroku pipelines.

## Disclaimer

This utility is beta, it may crash and / or behave unexpectedly, please do not use it on production
environments.

This project started as a side project to learn Rust, so there is a lot of areas to improve.

## Getting Started

### Prerequisites

You will need a version of the Rust Programming language (>= 1.27.0), it should come with Cargo, the Rust packet manager.
See the [Rust documentation](https://doc.rust-lang.org/cargo/getting-started/installation.html) for more details.

This project has only been tested on Linux (Debian based distro). Feel free to test it on another OS and share the outcomes.

### Installing

```bash
$ git clone git@gitlab.com:jveillet/heroku-env-rs.git
$ cd heroku-env-rs
$ cargo install
```

### Compile from source

**Note**: You can also build this project using Docker (see the Compile with Docker section).

#### Compile locally

```bash
$ git clone git@gitlab.com:jveillet/heroku-env-rs.git
$ cd heroku-env-rs
$ cargo build --release
$ cargo run -- push -c "my_dir/my_file.yml"
```

#### Compile with Docker

```bash
$ git clone git@gitlab.com:jveillet/heroku-env-rs.git
$ cd heroku-env-rs
$ docker-compose build
$ docker-compose run --rm app cargo build --release
$ docker-compose run --rm app cargo run -- push "my_dir/my_file.yml"
```

#### Rust formatting

As a best practice, this project uses Rust fmt to format the code, and comply with the [Rust
Styleguide](https://github.com/rust-lang-nursery/fmt-rfcs).

To run Rust fmt, either install it on your system with rustup:

```bash
$ cd heroku-env-rs
$ rustup component add rustfmt-preview
$ cargo fmt
```

Or use the Docker image built for this project.

```bash
$ cd heroku-env-rs
$ docker-compose run --rm app cargo fmt
```

##### Clippy

As of Rust 1.30, Clippy is now integrated with the stable version of Rust.

You can now install it via `rustup component add clippy-preview`.

You can also manually launch it via the command line:

```bash
$ cd heroku-env-rs/
$ cargo clippy
# OR with Docker Compose
$ docker-compose run --rm app cargo clippy
```

## Heroku Personal OAuth Token

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
$ cd heroku-env-rs/
$ touch .env
$ echo "HK_API_TOKEN=my_api_token" >> .env
```

Or you can export it as an environment variable into in your `~/.bashrc` or `~/.zshrc`.

```bash
$ export HK_API_TOKEN="my_api_token"
```

## Usage

`$ hke -h`

```bash
heroku-env-rs 0.1.6
J??r??mie Veillet <jeremie.veillet@gmail.com>
CLI to interact with config vars on Heroku written in Rust.

USAGE:
    hke [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    pull    Pull heroku config vars down to the local environment
    push    Push local config vars to heroku
```

### Push config vars

`$ hke push -h`

```bash
hke-push
Push local config vars to heroku

USAGE:
    hke push [OPTIONS] <KEY=VALUE>... --app <NAME>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --app <NAME>       App to run command against
    -c, --config <FILE>    Sets a user defined config file in YAML format

ARGS:
    <KEY=VALUE>...    Key-Value pairs of config vars
```

#### Push config vars for a single heroku app

You can set config vars for a single heroku app, by using a `-a` or `--app` option with the app name as the option value,
and pass the config vars in the form of KEY=VALUE separated by a whitespace bewteen each key-value pair.

```bash
$ hke push -a fuzzy-app MY_VAR=MY_VALUE
```

#### Push config vars for multiple heroku apps

The utility can use a configuration file in order to update the config vars on Heroku, for multiple apps at once.
This file can contain informations about the apps and the config vars values.

This file must be a YAML file, the tool will be looking for the file path passed by the command line option `-c` or `--config`.

```bash
$ hke push -c "/my_path/config.yml"
```

##### Definition of the YAML configuration file

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

### Pull config vars

`$ hke pull -h`

```bash
hke-pull
Pull heroku config vars down to the local environment

USAGE:
    hke pull [OPTIONS] --app <NAME>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --app <NAME>...    App to run command against
    -o, --output <FILE>    Save the output to a config file in YAML format
```

#### Pull a single heroku app

```bash
$ hke pull -a my-fuzzy-app
my-fuzzy-app
ENV=test
CLOUD_URL=https://www.github.com
-------------------------
```

#### Pull multiple heroku apps

```bash
$ hke pull -a my-fuzzy-app -a my-second-fuzzy-app
my-fuzzy-app
ENV=test
CLOUD_URL=https://www.github.com
-------------------------
my-second-fuzzy-app
ENV=test
CLOUD_URL=https://www.gitlab.com
-------------------------
```

#### Save output into a YAML file

```bash
$ hke pull -a my-fuzzy-app -o test.yml
my-fuzzy-app
ENV=test
CLOUD_URL=https://www.gitlab.com
-------------------------
Successfully created config file at test.yml
```

## Tests

Running tests:

```bash
$ cargo test
```

With Docker:

```bash
$ docker-compose run --rm app cargo test
```

## Contributing

You want to contribute to the Project? Yeah!! :v: ????  Contributors are always welcome! :thumbsup:

**Note**: One of the best ways to help right now is to use the utility and report issues!

### Bugs

If you find bugs, first go to the [issues page](https://gitlab.com/jveillet/heroku-env-rs/issues) and search if a related issue isn't listed there.

Create a new issue and insert any informations that can help to reproduce the observed behavior:
* Command context
* Stack trace
* Expected bahevior
* Current behavior
* OS / environment

Consider adding the ~bug label on your ticket.

### Feature requests

Create a new issue on the [issues page](https://gitlab.com/jveillet/heroku-env-rs/issues) and add a clear description of what the new feature should look like.

Consider adding the ~"feature request" label on your ticket.

### Pull Requests

1. Fork heroku-env
2. Clone your fork git clone `https://gitlab.com/$YOUR_USERNAME/heroku-env-rs && cd heroku-env-rs`
3. Create new branch `git checkout -b new-branch`
4. Make your changes, and commit `git commit -am "your message"`

## Licence.

heroku-env-rs is a free software: you can redistribute it and/or modify it under the terms of the [GNU GPL v3](LICENCE).

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see http://www.gnu.org/licenses/.


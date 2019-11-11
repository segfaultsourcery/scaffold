# Scaffold
### Quickly add dependencies to your existing Rust project.

I find myself always scouring the internet or looking through old projects to find the same dependencies over and over.
This is a tool I made to automate that process.

[Find it on crates.io](https://crates.io/crates/scaffold)

The help screen really says it all.

```text
scaffold 0.1.0
Quickly add dependencies to your Rust project.

USAGE:
    scaffold [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    

OPTIONS:
        --path <path>     [default: Cargo.toml]

SUBCOMMANDS:
    actix-web-server    Add actix-web, actix-files, and actix-session.
    cli                 Add structopt and config.
    crate               Add a crate.
    crates              Add multiple crates.
    csv                 Add serde, serde_derive, and csv.
    env-logger          Add env_logger.
    help                Prints this message or the help of the given subcommand(s)
    json                Add serde, serde_derive, and serde_json.
    regex               Add regex, and lazy_static.
    serde               Add serde, and serde_derive.
    toml                Add serde, serde_derive, and toml.
    web-client          Add reqwest.
```

## Examples:

Before:
```toml
[package]
name = "some_rust_project"
version = "0.1.0"
authors = ["Kim Hermansson <h4xrk1m@gmail.com>"]
edition = "2018"

[dependencies]
```

```bash
$ scaffold cli
$ scaffold json
```

After:
```toml
[package]
name = "empty-test-file"
version = "0.1.0"
authors = ["Kim Hermansson <h4xrk1m@gmail.com>"]
edition = "2018"

[dependencies]
config = "0.9.3"
serde_derive = "1.0.102"
serde_json = "1.0.41"
structopt = "0.3.4"

[dependencies.serde]
version = "1.0.102"
features = ["derive"]
```

Let's say I also want the `rand` crate:

```bash
$ scaffold crate rand
```

Or let's say I want the `rand` crate with the `std` feature:

```bash
$ scaffold crate rand --features std
```

Or maybe I want two extra crates:

```bash
$ scaffold crates rand regex
```
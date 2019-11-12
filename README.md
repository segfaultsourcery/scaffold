# Scaffold
### Quickly add dependencies to your existing Rust project.

I find myself always scouring the internet or looking through old projects to find the same dependencies over and over.
This is a tool I made to automate that process.

[Find it on crates.io](https://crates.io/crates/scaffold)

The help screen really says it all.

```text
scaffold 0.2.0
Quickly add dependencies to your Rust project.

USAGE:
    scaffold [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
    -a, --ask        Ask before each dependency.
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Be more verbose.

OPTIONS:
    -g, --groups <groups-path>     [default: ~/.config/scaffold/groups.toml]
    -p, --path <toml-path>         [default: Cargo.toml]

SUBCOMMANDS:
    add     Add groups to your project.
    help    Prints this message or the help of the given subcommand(s)
    list    List all available groups.
```

## Some outputs
### scaffold list

```bash
json
    serde_json = "1.0.41"
    serde_derive = "1.0.102"
    serde = { features = ["derive"], version = "1.0.102" }
cli
    config = "0.9.3"
    structopt = "0.3.4"
    shellexpand = "1.0.0"
```


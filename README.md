# Scaffold
## Quickly add dependencies to your existing Rust project.

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

### Define custom groups
By default, scaffold will look for groups in `~/.config/scaffold/groups.toml`.
_If this file doesn't exist, it will be created._

For the sake of convenience, `groups.toml` is a toml file, with the intent of looking and feeling like `Cargo.toml`.

Example:
```toml
[json]
serde_derive = "*"
serde_json = "*"
serde = { version = "*", features = ["derive"] }

[cli]
structopt = "*"
config = "*"
shellexpand = "*"
```

_Note that if the version is starred, then scaffold will try to determine the latest version._

### List available groups
You can list all your available groups:
```bash
$ scaffold list
```

Result:
```text
cli
    config = "0.9.3"
    shellexpand = "1.0.0"
    structopt = "0.3.4"
json
    serde = { features = ["derive"], version = "1.0.102" }
    serde_derive = "1.0.102"
    serde_json = "1.0.41"
```

### Add groups to your project
To add groups simply say:
```bash
$ scaffold --verbose add json
```

Result:
```text
Adding serde = { features = ["derive"], version = "1.0.102" }.
Adding serde_derive = "1.0.102".
Adding serde_json = "1.0.41".
```

You can add more than one at the same time:

```bash
$ scaffold --verbose add json cli
```

Result:
```text
Adding serde = { features = ["derive"], version = "1.0.102" }.
Adding serde_derive = "1.0.102".
Adding serde_json = "1.0.41".
Adding config = "0.9.3".
Adding shellexpand = "1.0.0".
Adding structopt = "0.3.4".
```

### Asking before inserting each crate
You can also tell it to ask you before each crate to see if you want it:
```bash
$ scaffold --ask --verbose add json cli
```

Result:
```text
Add config = "0.9.3"? [Y/n] y
Adding config = "0.9.3".
Add shellexpand = "1.0.0"? [Y/n] n
Add structopt = "0.3.4"? [Y/n] y
Adding structopt = "0.3.4".
```


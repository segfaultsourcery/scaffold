use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "scaffold",
    about = "Quickly add dependencies to your Rust project."
)]
pub(crate) struct Arguments {
    #[structopt(
        short = "p",
        long = "path",
        default_value = "Cargo.toml",
        parse(from_os_str)
    )]
    toml_path: PathBuf,

    #[structopt(
        short = "g",
        long = "groups",
        default_value = "~/.config/scaffold/groups.toml",
        parse(from_os_str)
    )]
    groups_path: PathBuf,

    /// Ask before each dependency.
    #[structopt(short, long, about = "Ask before adding each dependency.")]
    pub ask: bool,

    /// Be more verbose.
    #[structopt(short, long)]
    pub(crate) verbose: bool,

    #[structopt(subcommand)]
    pub(crate) subcommand: Subcommand,
}

#[derive(Debug, StructOpt)]
pub enum Subcommand {
    #[structopt(about = "List all available groups.")]
    List,
    #[structopt(about = "Add groups to your project.")]
    Add { group_names: Vec<String> },
}

impl Arguments {
    pub fn get_groups_path(&self) -> PathBuf {
        Arguments::tilde(&self.groups_path)
    }

    pub fn get_toml_path(&self) -> PathBuf {
        Arguments::tilde(&self.toml_path)
    }

    fn tilde(path: &PathBuf) -> PathBuf {
        PathBuf::from(shellexpand::tilde(path.to_str().unwrap()).to_string())
    }
}

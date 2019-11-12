use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "scaffold",
    about = "Quickly add dependencies to your Rust project."
)]
pub(crate) struct Arguments {
    #[structopt(short = "p", long = "path", default_value = "Cargo.toml")]
    pub(crate) toml_path: String,

    #[structopt(short, long, default_value = "default.toml")]
    pub(crate) groups_path: String,

    #[structopt(short, long, about = "Ask for each dependency.")]
    pub ask: bool,

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

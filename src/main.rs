use structopt::StructOpt;

mod toml_editor;
mod version_getter;

#[derive(Debug, StructOpt)]
enum Subcommand {
    Crate {
        name: String,

        #[structopt(long)]
        features: Vec<String>,
    },
    Crates {
        names: Vec<String>,
    },

    Json,
    Toml {
        #[structopt(long)]
        preserve_order: bool,
    },
    Csv,
    WebClient,
    Serde,
    Regex,
    EnvLogger,
    ActixWebServer,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "scaffold",
    about = "Quickly add dependencies to your Rust project."
)]
struct Arguments {
    #[structopt(long, default_value = "Cargo.toml")]
    path: String,

    #[structopt(short, long)]
    verbose: bool,

    #[structopt(subcommand)]
    subcommand: Subcommand,
}

fn main() {
    let arguments: Arguments = Arguments::from_args();
    let mut toml = toml_editor::read_toml_file(&arguments.path).unwrap();

    match arguments.subcommand {
        Subcommand::Crate { name, features } => {
            let features: Vec<&str> = features.iter().map(|feature| feature.as_str()).collect();
            toml.add_crate(&name, &features, arguments.verbose);
        }
        Subcommand::Crates { names } => {
            for name in names {
                toml.add_crate(&name, &[], arguments.verbose);
            }
        }
        Subcommand::Json => {
            toml.add_crate("serde", &["derive"], arguments.verbose);
            toml.add_crate("serde_json", &[], arguments.verbose);
            toml.add_crate("serde_derive", &[], arguments.verbose);
        }
        Subcommand::Toml { preserve_order } => {
            toml.add_crate("serde", &["derive"], arguments.verbose);
            toml.add_crate("serde_derive", &[], arguments.verbose);
            toml.add_crate(
                "toml",
                if preserve_order {
                    &["preserve_order"]
                } else {
                    &[]
                },
                arguments.verbose,
            );
        }
        Subcommand::Csv => {
            toml.add_crate("serde", &["derive"], arguments.verbose);
            toml.add_crate("serde_derive", &[], arguments.verbose);
            toml.add_crate("csv", &[], arguments.verbose);
        }
        Subcommand::WebClient => {
            toml.add_crate("reqwest", &[], arguments.verbose);
        }
        Subcommand::Serde => {
            toml.add_crate("serde", &["derive"], arguments.verbose);
            toml.add_crate("serde_derive", &[], arguments.verbose);
        }
        Subcommand::Regex => {
            toml.add_crate("regex", &[], arguments.verbose);
            toml.add_crate("lazy_static", &[], arguments.verbose);
        }
        Subcommand::EnvLogger => {
            toml.add_crate("env_logger", &[], arguments.verbose);
        }
        Subcommand::ActixWebServer => {
            toml.add_crate("actix-web", &[], arguments.verbose);
            toml.add_crate("actix-files", &[], arguments.verbose);
            toml.add_crate("actix-session", &[], arguments.verbose);
        }
    }

    toml.sort_dependencies();
    toml_editor::write_toml_file(&arguments.path, &toml).unwrap();
}

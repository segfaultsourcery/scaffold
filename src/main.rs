use std::str::FromStr;
use structopt::StructOpt;

mod toml_editor;
mod version_getter;

#[derive(Debug, StructOpt)]
enum Addition {
    Regex,
    Reqwest,
    Serde,
    SerdeDerive,
    SerdeJson,
    LazyStatic,
    Toml { preserve_order: bool },
    StructOpt,
    Rand,
    Base64,
}

impl FromStr for Addition {
    type Err = String;
    fn from_str(field: &str) -> Result<Self, Self::Err> {
        match field {
            "rand" => Ok(Addition::Rand),
            "base64" => Ok(Addition::Base64),
            "regex" => Ok(Addition::Regex),
            "reqwest" => Ok(Addition::Reqwest),
            "lazy_static" => Ok(Addition::LazyStatic),
            "serde_json" => Ok(Addition::SerdeJson),
            "serde_derive" => Ok(Addition::SerdeDerive),
            "serde" => Ok(Addition::Serde),
            "structopt" => Ok(Addition::StructOpt),
            "toml" => Ok(Addition::Toml {
                preserve_order: false,
            }),
            "toml-preserve-order" => Ok(Addition::Toml {
                preserve_order: true,
            }),

            _ => Err(format!("Invalid field: {:?}.", field)),
        }
    }
}

#[derive(Debug, StructOpt)]
enum Subcommand {
    #[structopt()]
    Add { additions: Vec<Addition> },
}

#[derive(Debug, StructOpt)]
#[structopt(name = "scaffold", about = "Quick edit your Rust project.")]
struct Arguments {
    #[structopt(long, default_value = "Cargo.toml")]
    path: String,

    #[structopt(subcommand)]
    subcommand: Subcommand,
}

fn main() {
    let arguments: Arguments = Arguments::from_args();
    let mut toml = toml_editor::read_toml_file(&arguments.path).unwrap();

    match arguments.subcommand {
        Subcommand::Add { additions } => {
            if additions.is_empty() {
                eprintln!("There's nothing to do.");
                return;
            }

            for addition in additions {
                match addition {
                    Addition::SerdeJson => toml_editor::add_serde_json(&mut toml),
                    Addition::Serde => toml_editor::add_serde(&mut toml),
                    Addition::StructOpt => toml_editor::add_structopt(&mut toml),
                    Addition::Toml { preserve_order } => {
                        toml_editor::add_toml(&mut toml, preserve_order)
                    }
                    Addition::Regex => toml_editor::add_regex(&mut toml),
                    Addition::Reqwest => toml_editor::add_reqwest(&mut toml),
                    Addition::LazyStatic => toml_editor::add_lazy_static(&mut toml),
                    Addition::Rand => toml_editor::add_rand(&mut toml),
                    Addition::Base64 => toml_editor::add_base64(&mut toml),
                    Addition::SerdeDerive => toml_editor::add_serde_derive(&mut toml),
                }
            }

            toml_editor::sort_dependencies(&mut toml);
            toml_editor::write_toml_file(&arguments.path, &toml).unwrap();
        }
    }
}

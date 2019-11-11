use std::str::FromStr;
use structopt::StructOpt;

mod toml_editor;

#[derive(Debug, StructOpt)]
enum Addition {
    Serde,
    Json,
    Toml,
    StructOpt,
}

impl FromStr for Addition {
    type Err = String;
    fn from_str(field: &str) -> Result<Self, Self::Err> {
        match field {
            "json" => Ok(Addition::Json),
            "serde" => Ok(Addition::Serde),
            "structopt" => Ok(Addition::StructOpt),
            "toml" => Ok(Addition::Toml),
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
                    Addition::Json => toml_editor::add_json(&mut toml),
                    Addition::Serde => toml_editor::add_serde(&mut toml),
                    Addition::StructOpt => toml_editor::add_structopt(&mut toml),
                    Addition::Toml => toml_editor::add_toml(&mut toml),
                }
            }

            toml_editor::sort_dependencies(&mut toml);

            println!("toml = {:#?}", toml);

            toml_editor::write_toml_file(&arguments.path, &toml).unwrap();
        }
    }
}

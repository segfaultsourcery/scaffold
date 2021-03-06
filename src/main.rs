use crate::arguments::Subcommand;
use crate::error::Error;
use crate::toml_editor::Config;
use crate::version_getter::VersionGetter;
use arguments::Arguments;
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::process::exit;
use structopt::StructOpt;

mod arguments;
mod default_toml;
mod error;
mod group_reader;
mod toml_editor;
mod version_getter;

fn main() {
    let arguments: Arguments = Arguments::from_args();
    if let Err(error) = ensure_groups_exist(&arguments) {
        eprintln!("Failed to create default scaffolding: {}", error);
        exit(1);
    }

    match &arguments.subcommand {
        Subcommand::List => {
            if let Err(error) = list_groups(&arguments) {
                eprintln!("Failed to read groups: {}", error);
                exit(1);
            }
        }
        Subcommand::Add { group_names } => {
            let mut toml = match toml_editor::read_toml_file(&arguments.get_toml_path()) {
                Ok(toml) => toml,
                Err(_) => {
                    eprintln!(
                        "Could not read TOML file at the specified path: {:?}.",
                        &arguments.get_toml_path()
                    );
                    exit(1);
                }
            };

            if let Err(error) = add_groups(&mut toml, &group_names, &arguments) {
                eprintln!("Could not add specified groups: {}", error);
                exit(1);
            }

            toml.sort_dependencies();
            if let Err(error) = toml_editor::write_toml_file(&arguments.get_toml_path(), &toml) {
                eprintln!("Could not write TOML file: {}", error);
                exit(1);
            }
        }
    }
}

fn ensure_groups_exist(arguments: &Arguments) -> Result<(), Error> {
    let path = arguments.get_groups_path();

    if path.exists() {
        return Ok(());
    }

    match path.parent() {
        None => {}
        Some(parent) => {
            if let Err(error) = create_dir_all(parent) {
                eprintln!("Failed to create default path: {}", error);
                exit(1);
            };
        }
    }

    match OpenOptions::new().write(true).create_new(true).open(&path) {
        Ok(mut file) => match file.write_all(default_toml::DEFAULT_TOML) {
            Ok(_) => {
                if arguments.verbose {
                    println!("Wrote default groups to {:?}.", &path);
                }
                Ok(())
            }
            Err(error) => Err(Error::Io(error)),
        },
        Err(error) => Err(Error::Io(error)),
    }
}

fn list_groups(arguments: &Arguments) -> Result<(), Error> {
    let groups = group_reader::get_groups(&arguments.get_groups_path())?;
    let mut version_getter = VersionGetter::default();

    for (name, dependencies) in groups {
        println!("{}", name);

        for dependency in dependencies {
            println!(
                "    {}",
                dependency.get_pretty_string(&mut version_getter, arguments.use_tilde_version)
            );
        }
    }

    Ok(())
}

fn add_groups(
    toml: &mut Config,
    group_names: &[String],
    arguments: &Arguments,
) -> Result<(), Error> {
    let groups = group_reader::get_groups(&arguments.get_groups_path())?;

    for group_name in group_names {
        match groups.get(group_name) {
            Some(group) => {
                for dependency in group {
                    toml.add_dependency(dependency, &arguments);
                }
            }
            None => return Err(Error::GroupNotFound(group_name.to_string())),
        }
    }

    Ok(())
}

use crate::arguments::Subcommand;
use crate::error::Error;
use crate::toml_editor::Config;
use crate::version_getter::VersionGetter;
use arguments::Arguments;
use structopt::StructOpt;

mod arguments;
mod error;
mod group_reader;
mod toml_editor;
mod version_getter;

fn main() {
    let arguments: Arguments = Arguments::from_args();

    match &arguments.subcommand {
        Subcommand::List => {
            list_groups(&arguments).expect("Failed to read groups.");
        }
        Subcommand::Add { group_names } => {
            let mut toml = toml_editor::read_toml_file(&arguments.toml_path).unwrap();

            for group_name in group_names {
                add_group(&mut toml, &group_name, &arguments).expect("Failure");
            }

            toml.sort_dependencies();
            toml_editor::write_toml_file(&arguments.toml_path, &toml).unwrap();
        }
    }
}

fn list_groups(arguments: &Arguments) -> Result<(), Error> {
    let groups = group_reader::get_groups(&arguments.groups_path)?;
    let mut version_getter = VersionGetter::default();

    for (name, dependencies) in groups {
        println!("{}", name);

        for dependency in dependencies {
            println!("    {}", dependency.get_pretty_string(&mut version_getter));
        }
    }

    Ok(())
}

fn add_group(toml: &mut Config, group_name: &str, arguments: &Arguments) -> Result<(), Error> {
    let groups = group_reader::get_groups(&arguments.groups_path)?;
    match groups.get(group_name) {
        Some(group) => {
            for dependency in group {
                toml.add_dependency(dependency, arguments.verbose);
            }
            Ok(())
        }
        None => Err(Error::GroupNotFound(group_name.to_string())),
    }
}

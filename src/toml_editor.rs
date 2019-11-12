use crate::error::Error;
use crate::group_reader::Dependency;
use crate::version_getter::VersionGetter;
use indexmap::IndexMap;
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::iter::FromIterator;
use std::path::PathBuf;
use toml::Value;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Config {
    pub package: IndexMap<String, Value>,
    pub dependencies: IndexMap<String, Value>,

    #[serde(flatten)]
    pub other: IndexMap<String, Value>,

    #[serde(skip)]
    version_getter: VersionGetter,
}

pub(crate) fn read_toml_file(path: &PathBuf) -> Result<Config, Error> {
    match File::open(path) {
        Ok(mut handle) => {
            let mut buffer = String::new();
            match handle.read_to_string(&mut buffer) {
                Ok(_) => match toml::from_str::<Config>(&buffer) {
                    Ok(config) => Ok(config),
                    Err(error) => Err(Error::TomlDe(error)),
                },
                Err(error) => Err(Error::Io(error)),
            }
        }
        Err(error) => Err(Error::Io(error)),
    }
}

pub(crate) fn write_toml_file(path: &PathBuf, config: &Config) -> Result<(), Error> {
    match toml::to_string(config) {
        Ok(buffer) => match File::create(&path) {
            Ok(mut handle) => match handle.write_all(buffer.as_bytes()) {
                Ok(_) => Ok(()),
                Err(error) => Err(Error::Io(error)),
            },
            Err(error) => Err(Error::Io(error)),
        },
        Err(error) => Err(Error::TomlSer(error)),
    }
}

impl Config {
    pub fn sort_dependencies(&mut self) {
        let mut dependencies = Vec::from_iter(&self.dependencies);

        dependencies.sort_by_key(|&(key, value)| {
            let weight = match value {
                Value::Table(_) => 1,
                _ => 0,
            };

            (weight, key)
        });

        let mut new_dependencies = IndexMap::new();
        for (key, value) in dependencies {
            new_dependencies.insert(key.to_string(), value.to_owned());
        }

        self.dependencies = new_dependencies;
    }
}

impl Config {
    pub fn add_dependency(&mut self, dependency: &Dependency, verbose: bool) {
        let name = dependency.name.to_string();
        let version = dependency.get_version_as_string(&mut self.version_getter);

        if verbose {
            println!(
                "{}",
                &dependency.get_pretty_string(&mut self.version_getter)
            );
        }

        match &dependency.other {
            None => {
                self.dependencies.insert(name, Value::String(version));
            }
            Some(other) => {
                if let Value::Table(mut other) = other.clone() {
                    other.insert("version".to_string(), Value::String(version));
                    let other = Value::Table(other);

                    self.dependencies.insert(name, other);
                }
            }
        };
    }
}

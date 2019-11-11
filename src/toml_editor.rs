use crate::version_getter::VersionGetter;
use indexmap::IndexMap;
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::iter::FromIterator;
use toml::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub package: IndexMap<String, Value>,
    pub dependencies: IndexMap<String, Value>,

    #[serde(flatten)]
    pub other: IndexMap<String, Value>,

    #[serde(skip)]
    version_getter: VersionGetter,
}

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    TomlSer(toml::ser::Error),
    TomlDe(toml::de::Error),
}

pub fn read_toml_file(path: &str) -> Result<Config, Error> {
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

pub fn write_toml_file(path: &str, config: &Config) -> Result<(), Error> {
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
    pub fn add_crate(&mut self, name: &str, features: &[&str], verbose: bool) {
        let name = name.to_string();
        let version = self.version_getter.get_crate_version(&name).unwrap();

        if features.is_empty() {
            if verbose {
                println!(r#"{} = "{}""#, name, version);
            }
            self.dependencies.insert(name, Value::String(version));
        } else {
            let features = features.to_vec();

            if verbose {
                println!(
                    r#"{} = {{ version = "{}", features = {:?} }}"#,
                    name, &version, &features
                );
            }

            self.dependencies.insert(
                name,
                toml::toml! {
                    version = version
                    features = features
                },
            );
        }
    }
}

use crate::version_getter::get_crate_version;
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

pub fn sort_dependencies(toml: &mut Config) {
    let mut dependencies = Vec::from_iter(&toml.dependencies);

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

    toml.dependencies = new_dependencies;
}

pub fn add_serde(toml: &mut Config) {
    let name = "serde".to_string();
    let version = get_crate_version(&name).unwrap();
    toml.dependencies.insert(
        name,
        toml::toml! {
            version = version
            features = ["derive"]
        },
    );
}

pub fn add_serde_derive(toml: &mut Config) {
    let name = "serde_derive".to_string();
    let version = get_crate_version(&name).unwrap();
    toml.dependencies.insert(name, Value::String(version));
}

pub fn add_serde_json(toml: &mut Config) {
    let name = "serde_json".to_string();
    let version = get_crate_version(&name).unwrap();
    toml.dependencies.insert(name, Value::String(version));
}

pub fn add_toml(toml: &mut Config, preserve_order: bool) {
    let name = "toml".to_string();
    let version = get_crate_version(&name).unwrap();

    if preserve_order {
        toml.dependencies.insert(
            name,
            toml::toml! {version = version
            features = ["preserve_order"]},
        );
    } else {
        toml.dependencies.insert(name, Value::String(version));
    }
}

pub fn add_structopt(toml: &mut Config) {
    let name = "structopt".to_string();
    let version = get_crate_version(&name).unwrap();
    toml.dependencies.insert(name, Value::String(version));
}

pub fn add_regex(toml: &mut Config) {
    let name = "regex".to_string();
    let version = get_crate_version(&name).unwrap();
    toml.dependencies.insert(name, Value::String(version));
}

pub fn add_reqwest(toml: &mut Config) {
    let name = "reqwest".to_string();
    let version = get_crate_version(&name).unwrap();
    toml.dependencies.insert(name, Value::String(version));
}

pub fn add_lazy_static(toml: &mut Config) {
    let name = "lazy_static".to_string();
    let version = get_crate_version(&name).unwrap();
    toml.dependencies.insert(name, Value::String(version));
}

pub fn add_rand(toml: &mut Config) {
    let name = "rand".to_string();
    let version = get_crate_version(&name).unwrap();
    toml.dependencies.insert(name, Value::String(version));
}

pub fn add_base64(toml: &mut Config) {
    let name = "base64".to_string();
    let version = get_crate_version(&name).unwrap();
    toml.dependencies.insert(name, Value::String(version));
}

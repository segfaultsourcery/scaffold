use serde_derive::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use toml::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub dependencies: BTreeMap<String, Value>,
    pub package: BTreeMap<String, Value>,

    #[serde(flatten)]
    pub other: BTreeMap<String, Value>,
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


pub fn add_serde(toml: &mut Config) {
    toml.dependencies.insert("serde".to_string(), toml::toml!{
        version = "1.0"
        features = ["derive"]
    });

    toml.dependencies.insert("serde_derive".to_string(), Value::String("1.0".to_string()));
}

pub fn add_json(toml: &mut Config) {
    add_serde(toml);
    toml.dependencies.insert("serde_json".to_string(), Value::String("1.0".to_string()));
}

pub fn add_toml(toml: &mut Config) {
    add_serde(toml);
    toml.dependencies.insert("toml".to_string(), Value::String("0.5".to_string()));
}

pub fn add_structopt(toml: &mut Config) {
    toml.dependencies.insert("structopt".to_string(), Value::String("0.3".to_string()));
}


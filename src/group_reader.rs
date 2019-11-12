use crate::error::Error;
use crate::version_getter::VersionGetter;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;
use toml::Value;

#[derive(Debug, Deserialize)]
struct RawDependencyGroups {
    #[serde(flatten)]
    groups: HashMap<String, RawDependencyGroup>,
}

#[derive(Debug, Deserialize)]
struct RawDependencyGroup {
    #[serde(flatten)]
    raw_dependencies: HashMap<String, Value>,
}

#[derive(Debug)]
struct DependencyGroups {
    groups: HashMap<String, Vec<Dependency>>,
}

#[derive(Debug)]
pub(crate) enum Version {
    Latest,
    Specific(String),
}

impl FromStr for Version {
    type Err = ();

    fn from_str(version: &str) -> Result<Self, Self::Err> {
        match version {
            "" | "*" => Ok(Version::Latest),
            specific => Ok(Version::Specific(specific.to_string())),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Dependency {
    pub name: String,
    pub version: Version,
    pub other: Option<Value>,
}

impl Dependency {
    fn from_value(name: &str, value: Value) -> Option<Self> {
        match value {
            Value::String(version) => {
                let name = name.to_string();
                let version = Version::from_str(&version).unwrap();
                let other = None;

                Some(Self {
                    name,
                    version,
                    other,
                })
            }
            Value::Table(table) => {
                let name = name.to_string();

                let version = match table.get("version").and_then(|it| it.as_str()) {
                    None => Version::Latest,
                    Some(version) => Version::from_str(version).unwrap(),
                };

                let mut table = table.clone();
                table.remove("version");

                Some(Self {
                    name,
                    version,
                    other: Some(Value::Table(table)),
                })
            }
            _ => None,
        }
    }
}

impl Dependency {
    pub fn get_version_as_string(&self, version_getter: &mut VersionGetter) -> String {
        match &self.version {
            Version::Latest => match version_getter.get_crate_version(&self.name) {
                None => {
                    eprintln!("Could not find version for crate {:?}.", &self.name);
                    "*".to_string()
                }
                Some(version) => version,
            },
            Version::Specific(version) => version.to_string(),
        }
    }

    pub fn get_pretty_string(&self, version_getter: &mut VersionGetter) -> String {
        let version = self.get_version_as_string(version_getter);

        match &self.other {
            None => format!("{} = {:?}", &self.name, version),
            Some(other) => {
                if let Value::Table(mut other) = other.clone() {
                    other.insert("version".to_string(), Value::String(version));
                    let other = format!("{}", Value::Table(other))
                        .lines()
                        .collect::<Vec<_>>()
                        .join(", ");

                    format!("{} = {{ {} }}", &self.name, other)
                } else {
                    panic!("Expected Value::Table, got something else.")
                }
            }
        }
    }
}

pub(crate) fn get_groups(path: &PathBuf) -> Result<HashMap<String, Vec<Dependency>>, Error> {
    let raw_dependency_groups = read_toml_file(path)?;

    let mut groups = HashMap::new();

    for (group_name, raw_group) in raw_dependency_groups.groups {
        for (dependency_name, raw_dependency) in raw_group.raw_dependencies {
            if let Some(dependency) = Dependency::from_value(&dependency_name, raw_dependency) {
                groups
                    .entry(group_name.to_string())
                    .or_insert_with(Vec::new)
                    .push(dependency);
            }
        }
    }

    for (_, group) in &mut groups {
        group.sort_by_key(|it| it.name.to_string());
    }

    Ok(groups)
}

fn read_toml_file(path: &PathBuf) -> Result<RawDependencyGroups, Error> {
    match File::open(path) {
        Ok(mut handle) => {
            let mut buffer = String::new();
            match handle.read_to_string(&mut buffer) {
                Ok(_) => match toml::from_str::<RawDependencyGroups>(&buffer) {
                    Ok(data) => Ok(data),
                    Err(error) => Err(Error::TomlDe(error)),
                },
                Err(error) => Err(Error::Io(error)),
            }
        }
        Err(error) => Err(Error::Io(error)),
    }
}

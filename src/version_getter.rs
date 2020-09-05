use std::collections::HashMap;
use std::iter::FromIterator;
use std::process::Command;

#[derive(Debug)]
pub(crate) struct VersionGetter {
    stored_versions: HashMap<String, String>,
}

impl Default for VersionGetter {
    fn default() -> Self {
        Self {
            stored_versions: Default::default(),
        }
    }
}

impl VersionGetter {
    pub fn get_crate_version(&mut self, name: &str, use_tilde_version: bool) -> Option<String> {
        if self.stored_versions.contains_key(name) {
            return self.stored_versions.get(name).cloned();
        }

        let result = Command::new("cargo")
            .args(vec!["search", name, "--limit", "1"])
            .output()
            .unwrap();

        assert!(result.status.success());

        let output = String::from_utf8(result.stdout).unwrap();

        match output.lines().next() {
            Some(line) => {
                let parts = Vec::from_iter(line.split_ascii_whitespace());

                let found_name = parts[0].to_string();
                let version = parts[2].replace("\"", "");

                if name == found_name {
                    self.stored_versions
                        .insert(name.to_string(), version.to_string());

                    if use_tilde_version {
                        Self::make_tilde_version(&version)
                    } else {
                        Some(version)
                    }
                } else {
                    None
                }
            }
            None => {
                self.stored_versions
                    .insert(name.to_string(), "*".to_string());
                None
            }
        }
    }

    fn make_tilde_version(version: &str) -> Option<String> {
        let version = version.rsplitn(2, '.');
        Some(format!("~{}", version.last()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        let version = VersionGetter::make_tilde_version("1.0.106");
        assert_eq!(version, Some("~1.0".to_string()));
    }
}

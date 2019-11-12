pub(crate) const DEFAULT_TOML: &[u8] = br#"[json]
serde_derive = "*"
serde_json = "*"
serde = { version = "*", features = ["derive"] }

[cli]
structopt = "*"
config = "*"
shellexpand = "*"
"#;

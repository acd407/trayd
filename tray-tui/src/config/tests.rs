use std::path::PathBuf;

use super::Config;

#[test]
fn default_has_no_socket_path() {
    let config = Config::default();
    assert!(config.socket_path.is_none());
}

#[test]
fn resolve_socket_prefers_cli_override() {
    let config = Config::default();
    let override_path = PathBuf::from("/tmp/custom.sock");
    let path = config.resolve_socket(Some(override_path.clone())).unwrap();
    assert_eq!(path, override_path);
}

#[test]
fn resolve_socket_uses_config_field() {
    let config = Config {
        socket_path: Some(PathBuf::from("/tmp/from-config.sock")),
    };
    let path = config.resolve_socket(None).unwrap();
    assert_eq!(path, PathBuf::from("/tmp/from-config.sock"));
}

#[test]
fn toml_parses_socket_path() {
    let toml = r#"socket_path = "/tmp/test.sock""#;
    let config: Config = toml::from_str(toml).unwrap();
    assert_eq!(config.socket_path.unwrap(), PathBuf::from("/tmp/test.sock"));
}

#[test]
fn toml_empty_uses_defaults() {
    let config: Config = toml::from_str("").unwrap();
    assert!(config.socket_path.is_none());
}

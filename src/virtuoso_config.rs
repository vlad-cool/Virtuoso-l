use serde;
use toml;

#[derive(Default, Debug, serde::Serialize, serde::Deserialize)]
pub struct VirtuosoConfig {
    pub legacy_backend: LegacyBackendConfig,
    pub cyrano_server: CyranoServerConfig,
}

impl VirtuosoConfig {
    const DEFAULT_PATH: &str = "config.toml";

    pub fn load_config(path: Option<String>) -> VirtuosoConfig {
        let mut loaded = true;
        let config =
            match std::fs::read_to_string(path.clone().unwrap_or(String::from(Self::DEFAULT_PATH)))
            {
                Ok(content) => match toml::from_str(&content) {
                    Ok(config) => config,
                    Err(err) => {
                        eprintln!("Error parsing config.toml: {}", err);
                        loaded = false;
                        VirtuosoConfig::default()
                    }
                },
                Err(err) => {
                    eprintln!("Error reading config.toml: {}", err);
                    loaded = false;
                    VirtuosoConfig::default()
                }
            };

        if !loaded {
            config.write_config(path);
        }

        config
    }

    pub fn write_config(&self, path: Option<String>) {
        let toml_str = toml::to_string(&self).expect("Failed to serialize config");
        std::fs::write(path.unwrap_or(String::from(Self::DEFAULT_PATH)), toml_str)
            .expect("Failed to write output.toml");
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LegacyBackendConfig {
    pub rc5_address: u32,
}

impl Default for LegacyBackendConfig {
    fn default() -> Self {
        Self { rc5_address: 32 }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CyranoServerConfig {
    pub cyrano_port: u16,
}

impl Default for CyranoServerConfig {
    fn default() -> Self {
        Self { cyrano_port: 50100 }
    }
}

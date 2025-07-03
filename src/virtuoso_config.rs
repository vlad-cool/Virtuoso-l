use serde;
use toml;

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VirtuosoConfig {
    #[serde(default)]
    pub legacy_backend: LegacyBackendConfig,
    #[serde(default)]
    pub cyrano_server: CyranoServerConfig,
    #[serde(default)]
    pub logger_config: LoggerConfig,
}

impl VirtuosoConfig {
    const DEFAULT_PATH: &str = "config.toml";

    pub fn load_config(path: Option<String>) -> VirtuosoConfig {
        let mut loaded: bool = true;
        let config: VirtuosoConfig =
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
        let toml_str: String = toml::to_string(&self).expect("Failed to serialize config");
        std::fs::write(path.unwrap_or(String::from(Self::DEFAULT_PATH)), toml_str)
            .expect("Failed to write output.toml");
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LegacyBackendConfig {
    pub rc5_address: u32,
}

impl Default for LegacyBackendConfig {
    fn default() -> Self {
        Self { rc5_address: 0 }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CyranoServerConfig {
    pub cyrano_port: u16,
}

impl Default for CyranoServerConfig {
    fn default() -> Self {
        Self { cyrano_port: 50100 }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum LogLevelOption {
    #[serde(alias="ALL", alias="all")]
    All,
    #[serde(alias="DEBUG", alias="debug")]
    Debug,
    #[serde(alias="INFO", alias="info")]
    Info,
    #[serde(alias="WARNING", alias="warning")]
    Warning,
    #[serde(alias="ERROR", alias="error")]
    Error,
    #[serde(alias="CRITICAL", alias="critical")]
    Critical,
    #[serde(alias="NONE", alias="none")]
    None,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LoggerConfig {
    pub log_level: Option<LogLevelOption>,
    pub log_path: Option<String>,
    #[serde(default)]
    pub stderr: bool,
    #[serde(default)]
    pub udp: bool,
    pub udp_port: Option<u16>,
    pub udp_ip: Option<String>,
    #[serde(default)]
    pub udp_print_ip: bool,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            log_level: None,
            log_path: None,
            stderr: false,
            udp: false,
            udp_port: None,
            udp_ip: None,
            udp_print_ip: false,
        }
    }
}

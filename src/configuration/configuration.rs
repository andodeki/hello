use serde_aux::field_attributes::deserialize_number_from_string;
// use secrecy::{ExposeSecret, Secret};

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
    // pub email_client: EmailClientSettings,
    // pub redis_uri: Secret<String>,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub name: String,
    // pub base_url: String,

    pub hmac_secret: String,
    // pub hmac_secret: Secret<String>,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    // pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

impl Settings{
    pub fn get_configuration() -> Result<Settings, config::ConfigError> {
        let base_path = std::env::current_dir().expect("Failed to determine the current directory");
        let configuration_directory = base_path.join("configuration");
    
        let settings = config::Config::builder()
            .add_source(config::File::from(configuration_directory.join("base.yaml")))
            // .add_source(config::File::from(configuration_directory.join(&environment_filename)))
            // // Add in settings from environment variables (with a prefix of APP and '__' as separator)
            // // E.g. `APP_APPLICATION__PORT=5001 would set `Settings.application.port`
            // .add_source(config::Environment::with_prefix("APP").prefix_separator("_").separator("__"))
            .build()?;
        
        settings.try_deserialize::<Settings>()
    }
    pub fn getName(self) -> String {
        // self.Hi = "Hello World".to_string();
        format!("{}", self.application.name)
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    let settings = config::Config::builder()
        .add_source(config::File::from(configuration_directory.join("base.yaml")))
        // .add_source(config::File::from(configuration_directory.join(&environment_filename)))
        // // Add in settings from environment variables (with a prefix of APP and '__' as separator)
        // // E.g. `APP_APPLICATION__PORT=5001 would set `Settings.application.port`
        // .add_source(config::Environment::with_prefix("APP").prefix_separator("_").separator("__"))
        .build()?;
    
    settings.try_deserialize::<Settings>()
}
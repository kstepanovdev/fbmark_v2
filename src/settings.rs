use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub tagpacker: Tagpacker,
}

#[derive(Debug, Deserialize)]
pub struct Tagpacker {
    pub user_id: String,
}

impl Settings {
    pub fn get_configuration() -> Result<Settings> {
        let base_path = std::env::current_dir().expect("Failed to determine the current directory");

        let settings = config::Config::builder()
            .add_source(config::File::from(base_path.join("config.yaml")))
            .build()?;

        Ok(settings.try_deserialize::<Settings>()?)
    }
}

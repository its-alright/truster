mod app_config;

use app_config::AppConfig;
use config::ConfigError;

fn main() {
    match load_config() {
        Ok(conf) => {
            let t = 0;

            //println!("{:#?}", conf);
            println!("done parsing");
        }
        Err(err) => {
            println!("Failed to load config: {}", err);
        }
    }
}

fn load_config() -> Result<AppConfig, ConfigError> {
    let config = config::Config::builder()
        .add_source(config::File::with_name("profiles/profile1.toml").required(true))
        .add_source(config::Environment::with_prefix("TRUST"))
        .build()?
        .try_deserialize()?;

    Ok(config)
}

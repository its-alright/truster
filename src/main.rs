mod app_config;
mod app_logger;

use app_config::{AppConfig, Args};
use app_logger::init_logger;

use clap::Parser;
use config::ConfigError;

use std::{collections::HashMap, sync::Arc};

/*
use std::{
    collections::HashMap,
    sync::{Arc, atomic::AtomicBool, atomic::Ordering},
    time::Duration,
};
 */

fn main() {
    let args = Args::parse();

    init_logger(&args);

    tracing::info!(
        profile = args.profile,
        duration = args.duration,
        log = args.log,
        "parse profile config"
    );

    match load_config(&args) {
        Ok(conf) => {
            let c = Arc::new(conf);
            tracing::info!("config parsed");
        }
        Err(err) => {
            tracing::error!("Failed to load config: {}", err);
            return;
        }
    }
}

fn load_config(args: &Args) -> Result<AppConfig, ConfigError> {
    let profile_path = args.profile.as_str();

    let config = config::Config::builder()
        .add_source(config::File::with_name(profile_path).required(true))
        .add_source(config::Environment::with_prefix("TRUST"))
        .build()?
        .try_deserialize()?;

    Ok(config)
}

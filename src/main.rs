mod app_config;
mod app_logger;
mod data_sources;
mod sources_uids_csv;

use app_config::{Args, load_config};
use app_logger::init_logger;

use clap::Parser;
use std::{
    collections::HashMap,
    sync::{Arc, atomic::AtomicBool, atomic::Ordering},
    time::Duration,
};

use tokio::time::{MissedTickBehavior, interval};

use crate::{app_config::SourceType, data_sources::DataSource, sources_uids_csv::load_from_csv};

/*
use std::{
    collections::HashMap,
    sync::{Arc, atomic::AtomicBool, atomic::Ordering},
    time::Duration,
};
 */

#[tokio::main]
async fn main() {
    let args = Args::parse();

    init_logger(&args);

    tracing::info!(
        profile = args.profile,
        duration = args.duration,
        log = args.log,
        "parse profile config"
    );

    let cfg = match load_config(&args) {
        Ok(conf) => {
            tracing::info!("config parsed");
            Arc::new(conf)
        }
        Err(err) => {
            tracing::error!("Failed to load config: {}", err);
            return;
        }
    };

    // let mut handles = vec![];

    //load all static sources
    let mut sources: HashMap<String, Box<dyn DataSource>> = HashMap::new();

    for cfg_source in cfg.sources.iter() {
        tracing::info!(name = cfg_source.name, "loading start...");

        match (&cfg_source.kind, cfg_source.name.as_str()) {
            (SourceType::Csv, "uids") => {
                let key = cfg_source.name.clone();
                //TODO Result pattern
                let source = load_from_csv(&cfg_source.value);

                tracing::info!(
                    name = cfg_source.name,
                    count = source.len(),
                    "loading successfull"
                );
                
                sources.insert(key, source);
            }
            _ => {
                tracing::warn!(name = cfg_source.name, "unknown source kind or name")
            }
        };

        tracing::info!(name = cfg_source.name, "loading done!");
    }

    tracing::info!("processing...");

    shutdown(args.duration).await;

    // Останавливаем всех воркеров
    tracing::info!("stopping workers...");

    // Ждём завершения
    // for handle in handles {
    //      let _ = handle.await;
    // }

    tracing::info!("done!");
}

async fn shutdown(duration: u64) {
    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_ctrl_clone = Arc::clone(&shutdown);
    // Настройка обработки Ctrl+C
    ctrlc::set_handler(move || {
        shutdown_ctrl_clone.store(true, Ordering::Relaxed);
        tracing::warn!("ctrl+c, shutdown...");
    })
    .expect("Ctrl+C error logic");

    // Ждём либо таймаут, либо Ctrl+C
    tokio::select! {
        _ = tokio::time::sleep(Duration::from_secs(duration)) => {
            tracing::info!("test duration reached, stopping...");
            shutdown.store(true, Ordering::Relaxed);
        }
        _ = async {
            // Ждём пока shutdown не станет true
            while !shutdown.load(Ordering::Relaxed) {
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        } => {
            tracing::info!("shutdown signal received");
        }
    }
}

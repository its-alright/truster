use tracing_subscriber::{
    EnvFilter,
    fmt::{self},
    prelude::*,
};

use crate::app_config::Args;

pub fn init_logger(args: &Args) {
    // 1. Настраиваем фильтр уровней логов (берем из переменной окружения RUST_LOG или ставим info по умолчанию)
    let filter = format!(
        "{},sqlx=warn,truster={}",
        args.log.to_string(),
        args.log.to_string()
    );
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(filter));

    // 2. Инициализируем слой JSON-форматированияinfo
    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt::layer().json()) // Этот слой превращает ВСЕ события tracing в JSON-строки
        .init();

    tracing::info!("logger inited");
}

use clap::Parser;
use config::ConfigError;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub name: String,
    pub base_url: String,
    pub endpoints: Vec<EndpointConfig>,
    pub sources: Vec<SourceConfig>,
}

#[derive(Deserialize, Debug)]
pub struct EndpointConfig {
    pub url: String,
    pub method: HttpMethodType,
    pub name: String,
    pub rps: u32,
    //#body/source
    pub source: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(try_from = "String")]
pub enum HttpMethodType {
    POST,
    GET,
}

impl TryFrom<String> for HttpMethodType {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        HttpMethodType::from_str(&value)
    }
}

impl FromStr for HttpMethodType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "post" => Ok(HttpMethodType::POST),
            "get" => Ok(HttpMethodType::GET),
            _ => Err(format!("Unknown http method: {}", s)),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct SourceConfig {
    pub name: String,
    pub kind: SourceType,
    pub value: String,
    custom_type: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(try_from = "String")]
pub enum SourceType {
    File,
    Const,
    Dynamic,
    Custom,
    Csv,
}

impl TryFrom<String> for SourceType {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        SourceType::from_str(&value)
    }
}

impl FromStr for SourceType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "const" => Ok(SourceType::Const),
            "custom" => Ok(SourceType::Custom),
            "dynamic" => Ok(SourceType::Dynamic),
            "file" => Ok(SourceType::File),
            "csv" => Ok(SourceType::Csv),
            _ => Err(format!("Unknown source type: {}", s)),
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Load testing utility", long_about = None)]
pub struct Args {
    //path to profile
    #[arg(short, long)]
    pub profile: String,

    /// Test duration in seconds
    #[arg(short, long, default_value = "300")]
    pub duration: u64,

    #[arg(short, long, default_value = "info")]
    pub log: String,
}

pub fn load_config(args: &Args) -> Result<AppConfig, ConfigError> {
    let profile_path = args.profile.as_str();

    let config = config::Config::builder()
        .add_source(config::File::with_name(profile_path).required(true))
        .add_source(config::Environment::with_prefix("TRUST"))
        .build()?
        .try_deserialize()?;

    Ok(config)
}

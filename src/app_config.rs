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
    name: String,
    kind: SourceType,
    value: String,
    custom_type: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(try_from = "String")]
pub enum SourceType {
    File,
    Const,
    Dynamic,
    Custom,
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
            "File" => Ok(SourceType::File),
            _ => Err(format!("Unknown source type: {}", s)),
        }
    }
}

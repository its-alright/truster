use serde::Deserialize;
use std::sync::RwLock;
use std::{collections::HashMap, sync::Arc};
use uuid::Uuid;

use anyhow::Result;

#[derive(Clone, Debug)]
pub enum DataItem {
    Uid(UidStatusParams),
    RequestId(RequestStatusParams),
}

#[derive(Clone, Debug, Deserialize)]
pub struct UidStatusParams {
    pub document_type: u8,
    pub uid: Uuid,
    pub operator_id: Uuid,
    pub request_type: u8,
    pub is_all_statuses: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RequestStatusParams {
    pub document_type: u8,
    pub request_id: Uuid,
    pub operator_id: Uuid,
    pub request_type: u8,
    pub is_all_statuses: bool,
}

// ---------- Общий трейт ----------
pub trait DataSource: Send + Sync {
    fn get_rnd(&self) -> Result<Option<DataItem>>;
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn push(&self, item: DataItem);
}

pub struct DataSourceHashMap {
    map: RwLock<HashMap<String, Arc<dyn DataSource + Send + Sync>>>,
}

impl DataSourceHashMap {
    pub fn new() -> Self {
        Self {
            map: RwLock::new(HashMap::new()),
        }
    }

    pub fn get(&self, key: &str) -> Option<Arc<dyn DataSource + Send + Sync>> {
        let guard = self.map.read().unwrap();
        guard.get(key).cloned() // Arc клонируется легко
    }

    pub fn insert(&self, key: String, value: Arc<dyn DataSource + Send + Sync>) {
        let mut guard = self.map.write().unwrap();
        guard.insert(key, value);
    }
}

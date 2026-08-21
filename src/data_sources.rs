use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug)]
pub enum DataItem<'a> {
    Uid(&'a UidStatusParams),
    // ...
}

// ---------- Общий трейт ----------
pub trait DataSource {
    fn get_rnd(&self) -> Option<DataItem>;
    fn is_empty(&self) -> bool;
    fn len(&self) -> u32;
}

// Имена полей должны соответствовать заголовкам CSV
#[derive(Debug, Deserialize)]
pub struct UidStatusParams {
    pub documentType: u8,
    pub uid: Uuid,
    pub operatorId: Uuid,
    pub requestType: u8,
    pub isAllStatuses: bool,
}

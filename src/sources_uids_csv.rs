use crate::data_sources::{DataItem, DataSource, UidStatusParams};
use anyhow::Result;
use anyhow::anyhow;
use rand::{seq::SliceRandom, thread_rng};
use std::any::type_name;
use std::sync::Mutex;

struct UidsSource {
    uids: Mutex<Vec<UidStatusParams>>,
}

impl UidsSource {
    pub fn new(v: Vec<UidStatusParams>) -> Self {
        Self {
            uids: Mutex::new(v),
        }
    }
}

impl DataSource for UidsSource {
    fn get_rnd(&self) -> Result<Option<DataItem>> {
        let mut rng = thread_rng();
        let map_guard = self
            .uids
            .lock()
            .map_err(|e| anyhow!("Failed to lock mutex: {}", e))?;

        // Выбираем случайный элемент
        match map_guard.choose(&mut rng) {
            Some(item) => {
                // Возвращаем клон преобразованный в DataItem
                Ok(Some(DataItem::Uid(item.clone())))
            }
            None => {
                // Вектор пустой - возвращаем None
                Err(anyhow!("DataSource has no items"))
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.uids
            .lock()
            .map(|guard| guard.is_empty())
            .unwrap_or(false)
    }

    fn len(&self) -> usize {
        self.uids.lock().map(|guard| guard.len()).unwrap_or(0)
    }

    fn push(&self, item: DataItem) {
        // Извлекаем UidStatusParams из enum
        let params = match item {
            DataItem::Uid(params) => params,
            _ => {
                let type_name = type_name::<UidsSource>();
                tracing::warn!(
                    source_type = type_name,
                    "failed to add Item to Uids DataSource..."
                );

                return;
            }
        };

        let mut guard = self.uids.lock().unwrap();
        guard.push(params);
    }
}

pub fn load_from_csv(path: &str) -> impl DataSource + use<> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path(path)
        //.from_path("./profiles/debug_0_uids.csv")
        .expect("failed parsing csv");

    let mut vec = Vec::new();
    // Перебираем записи, преобразуя их в структуру Record
    for result in rdr.deserialize() {
        let item: UidStatusParams = result.expect("failed parsing csv row");
        vec.push(item);
    }

    UidsSource::new(vec)
}

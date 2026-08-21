use crate::data_sources::{DataItem, DataSource, UidStatusParams};
use rand::{seq::SliceRandom, thread_rng};

struct UidsSource {
    uids: Vec<UidStatusParams>,
}

impl UidsSource {
    pub fn new(v: Vec<UidStatusParams>) -> Self {
        Self { uids: v }
    }
}

impl DataSource for UidsSource {
    fn get_rnd(&self) -> Option<DataItem> {
        let mut rng = thread_rng();
        self.uids.choose(&mut rng).map(|item| DataItem::Uid(item))
    }

    fn is_empty(&self) -> bool {
        self.uids.is_empty()
    }

    fn len(&self) -> u32 {
        self.uids.len() as u32
    }
}

pub fn load_from_csv(path: &str) -> Box<dyn DataSource> {
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

    Box::new(UidsSource::new(vec))
}

use std::{collections::HashMap, fmt::Arguments};

use fern::FormatCallback;
use log::Record;

#[derive(Debug, Default)]
pub(crate) struct KVVisitor {
    pub map: HashMap<String, String>,
}

impl std::fmt::Display for KVVisitor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.map)
    }
}

impl<'kvs> log::kv::VisitSource<'kvs> for KVVisitor {
    fn visit_pair(
        &mut self,
        key: log::kv::Key<'kvs>,
        value: log::kv::Value<'kvs>,
    ) -> Result<(), log::kv::Error> {
        self.map.insert(key.to_string(), value.to_string());
        Ok(())
    }
}

// The formatting function used by the log tauri plugin
pub(crate) fn format_log(out: FormatCallback, message: &Arguments, record: &Record) {
    let mut key_values = KVVisitor::default();

    record.key_values().visit(&mut key_values).unwrap();

    out.finish(format_args!(
        "[{}] [{}] [{}] {} {}",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        record.target(),
        record.level(),
        message,
        key_values,
    ))
}

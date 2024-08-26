use std::{collections::HashMap, fmt::Arguments};

use fern::FormatCallback;
use log::Record;

/// A visitor that collects key-value pairs from a log record for display.
#[derive(Debug, Default)]
pub(crate) struct KVVisitor {
    pub map: HashMap<String, String>,
}

// Implement the `Display` trait for the `KVVisitor` struct in order to print the key-value pairs
impl std::fmt::Display for KVVisitor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = Vec::<String>::new();
        for (key, value) in &self.map {
            lines.push(format!("    {}: {}", key, value));
        }

        let output = lines.join("\n");

        write!(f, "{}", output)
    }
}

/// Implement the `VisitSource` trait for the `KVVisitor` struct in order to read the key-value
/// pairs from a log record.
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

    // Format differently depending on if any key-value pairs are included in the record
    if key_values.map.len() > 0 {
        out.finish(format_args!(
            "{} {} [{}] {} \n{}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            record.target(),
            message,
            key_values,
        ));

        return;
    }

    out.finish(format_args!(
        "{} {} [{}] {}",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        record.level(),
        record.target(),
        message,
    ))
}

use std::collections::HashMap;
use std::time::SystemTime;

pub enum Field {
    Number(i32),
    String(String),
}

pub enum FieldType {
    Number,
    String,
}

pub struct Log<'a> {
    timestamp: SystemTime,
    log: HashMap<&'a str, Field>,
}

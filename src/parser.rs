use crate::commons::FieldType;
use crate::commons::Log;
use crate::sink::SinkConfig;
use std::collections::HashMap;
use std::path::Path;
use std::str::Bytes;

pub struct Config<'a> {
    source: Source<'a>,
    structure: Structure<'a>,
    delimiter: &'a str,
    sink_config: Vec<SinkConfig>,
}

pub struct Structure<'config> {
    regular_expression_map: HashMap<&'config str, &'config str>,
    type_map: HashMap<&'config str, FieldType>,
}

pub enum Source<'a> {
    FileType(&'a Path),
    IOType,
}

pub struct LogStream<'a> {
    bytes: ByteSplit<'a>,
    structure: Structure<'a>,
}

impl<'a> Iterator for LogStream<'a> {
    type Item = Log<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        //TODO:
        todo!()
    }
}

struct ByteSplit<'a> {
    bytes: Bytes<'a>,
    config: &'a Config<'a>,
}

impl<'a> ByteSplit<'a> {
    fn from(source: &Source, delimeter: &'a str) -> Self {
        //TODO :
        todo!();
    }
}

impl<'config> Iterator for ByteSplit<'config> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

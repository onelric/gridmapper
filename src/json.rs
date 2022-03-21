use std::{fs::File, path::PathBuf};

use serde_json::*;

pub fn load_json(path: &str) -> Value {
    let pathbuf = PathBuf::from(path);
    let json =
        serde_json::from_reader(File::open(pathbuf.with_extension("json")).unwrap()).unwrap();
    json
}

pub fn write_json(path: &str, data: Value) {
    let pathbuf = PathBuf::from(path);

    serde_json::to_writer_pretty(File::create(pathbuf.with_extension("json")).unwrap(), &data)
        .unwrap()
}

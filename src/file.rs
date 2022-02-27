use std::{io::Write, path::Path, str};

use chrono::{Datelike, Local};

/// Represents the filename of a Porquinho bookkeeping file
pub struct BookkeepingFile {
    name: [u8; 7],
}

impl BookkeepingFile {
    /// The bookkeeping file for this month
    /// E.g. if we're in October of 2024, the relevant file in which
    /// we'll record income and expenses is `10-2024`
    pub fn current_file() -> Self {
        let mut buf = [0; 7];

        let today = Local::today();
        let month = today.month();
        let year = today.year();

        // Safety: should not fail until after the year 9999
        write!(&mut buf[..], "{:02}-{year}", month).unwrap();

        Self { name: buf }
    }

    pub fn as_path(&self) -> &Path {
        // Safety: `current_file` must never make `self.name` be invalid UTF-8
        let filename = str::from_utf8(&self.name).unwrap();

        Path::new(filename)
    }
}
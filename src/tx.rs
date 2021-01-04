use anyhow::{Context, Error};
use fs::DirEntry;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use stringreader::StringReader;

fn connect(entry: DirEntry) -> Result<Option<UnixStream>, Error> {
    let path = entry.path();
    if path.is_dir() {
        Ok(None)
    } else {
        let stream = UnixStream::connect(path)?;
        Ok(Some(stream))
    }
}

fn get_streams(dir: &PathBuf) -> Result<Vec<UnixStream>, Error> {
    let streams = fs::read_dir(dir)?
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|entry| connect(entry).unwrap_or_default())
        .filter_map(|stream| stream);

    Ok(streams.collect())
}

pub fn write(dir: &PathBuf, value: Option<String>) -> Result<(), Error> {
    let mut last_error = None;

    let mut streams = get_streams(dir)?;

    let read: Box<dyn Read> = if let Some(v) = value.as_ref() {
        let reader = StringReader::new(v.as_str());
        Box::new(reader)
    } else {
        let stdin = io::stdin();
        Box::new(stdin)
    };

    let reader = BufReader::new(read);

    for line in reader.lines().filter_map(|line| line.ok()) {
        for stream in &mut streams {
            if let Err(err) = writeln!(stream, "{}", line) {
                last_error = Some(err);
            };
        }
    }

    if let Some(err) = last_error {
        Err(err).context("Unable to write to channel")
    } else {
        Ok(())
    }
}

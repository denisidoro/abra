use anyhow::Error;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::env;
use std::path::PathBuf;
use std::str;

fn random_id() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(5)
        .map(char::from)
        .collect()
}

pub fn dir_path(name: &str) -> Result<PathBuf, Error> {
    let tmp_dir = env::temp_dir();
    let dir = tmp_dir.join("com.github.denisidoro.abra").join(name);

    Ok(dir)
}

pub fn paths(name: &str) -> Result<(PathBuf, PathBuf), Error> {
    let dir = dir_path(name)?;
    let id = random_id();
    let socket = dir.join(id);

    Ok((dir, socket))
}

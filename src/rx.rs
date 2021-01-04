use crate::terminal;
use anyhow::{Context, Error};
use std::fs;
use std::io::{BufRead, BufReader};
use std::os::unix::net::UnixListener;
use std::path::PathBuf;
use std::process::Command;

pub struct Channel {
    path: PathBuf,
    pub listener: UnixListener,
}

impl Channel {
    pub fn bind(path: PathBuf) -> std::io::Result<Self> {
        UnixListener::bind(&path).map(|listener| Channel { path, listener })
    }

    pub fn read(&self, cmd: Option<&String>, should_clear: bool) -> Result<(), Error> {
        let mut last_error = None;

        for stream in self.listener.incoming() {
            if let Ok(s) = stream {
                BufReader::new(s)
                    .lines()
                    .filter_map(|line| line.ok())
                    .for_each(|line| {
                        if should_clear {
                            terminal::clear();
                        }

                        match cmd {
                            Some(cmd) => {
                                if let Err(err) = process(cmd, line.as_str()) {
                                    last_error = Some(err);
                                };
                            }

                            None => println!("{}", line),
                        }
                    });
            }
        }

        if let Some(err) = last_error {
            Err(err).context("Unable to process line")
        } else {
            Ok(())
        }
    }
}

impl Drop for Channel {
    fn drop(&mut self) {
        close(&self.path)
    }
}

pub fn close(path: &PathBuf) {
    let _ = fs::remove_file(&path);
}

fn process(cmd: &str, line: &str) -> Result<(), Error> {
    let interpolated_cmd = cmd.replace("{}", line);

    let output = Command::new("bash")
        .arg("-c")
        .arg(interpolated_cmd)
        .output()?;

    if !output.status.success() {
        anyhow!("Command executed with failing error code");
    }

    String::from_utf8(output.stdout)?
        .lines()
        .for_each(|line| println!("{}", line));

    Ok(())
}

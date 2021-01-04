use crate::filesystem;
use crate::opts::RxTxOpts;
use crate::rx::{close, Channel};
use anyhow::{Context, Error};
use std::fs;
use std::process;

pub fn main(opts: RxTxOpts) -> Result<(), Error> {
    let (dirpath, socketpath) = filesystem::paths(opts.channel.as_str())?;
    let p = socketpath.clone();

    ctrlc::set_handler(move || {
        close(&p);
        process::exit(130);
    })
    .expect("Error setting Ctrl-C handler");

    fs::create_dir_all(&dirpath)?;
    let channel = Channel::bind(socketpath).context("Unable to bind socket")?;

    channel.read(opts.cmd.as_ref(), opts.clear)
}

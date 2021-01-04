use anyhow::Error;

use crate::filesystem::dir_path;
use crate::opts::RxTxOpts;
use crate::tx::write;

pub fn main(opts: RxTxOpts) -> Result<(), Error> {
    let dir = dir_path(opts.channel.as_str())?;
    write(&dir, opts.value)
}

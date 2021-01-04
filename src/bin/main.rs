extern crate abra;

use abra::{handle_opts, opts_from_env, FileAnIssue};
use anyhow::Error;

fn main() -> Result<(), Error> {
    handle_opts(opts_from_env()).map_err(|e| FileAnIssue::new(e).into())
}

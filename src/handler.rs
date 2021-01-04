use crate::cmds;
use crate::opts::Command::{Faketty, Hook, Rx, Tx};
use crate::opts::Opts;
use anyhow::Error;

pub fn handle_opts(opts: Opts) -> Result<(), Error> {
    match opts.cmd {
        Rx(o) => cmds::rx::main(o),
        Tx(o) => cmds::tx::main(o),
        Hook { shell } => cmds::hook::main(shell),
        Faketty { cmd } => cmds::faketty::main(cmd.as_str()),
    }
}

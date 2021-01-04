use crate::opts::Shell::{self, Bash, Zsh};
use anyhow::Error;

pub fn main(shell: Shell) -> Result<(), Error> {
    let code = match shell {
        Bash => include_str!("../../shell/hook.bash"),
        Zsh => include_str!("../../shell/hook.zsh"),
    };

    println!("{}", code);

    Ok(())
}

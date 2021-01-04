use anyhow::Error;

pub fn main(cmd: &str) -> Result<(), Error> {
    let output = fake_tty::bash_command(cmd).output()?;
    let stdout = fake_tty::get_stdout(output.stdout)?;
    println!("{}", stdout);
    Ok(())
}

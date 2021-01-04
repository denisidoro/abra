use clap::{crate_version, AppSettings, Clap};
use std::str::FromStr;

pub enum Shell {
    Bash,
    Zsh,
}

// Implement the trait
impl FromStr for Shell {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bash" => Ok(Shell::Bash),
            "zsh" => Ok(Shell::Zsh),
            _ => Err("no match"),
        }
    }
}

#[derive(Clap)]
#[clap(setting = AppSettings::ColorAuto)]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(setting = AppSettings::AllowLeadingHyphen)]
#[clap(version = crate_version!())]
#[clap(after_help = r#"MORE INFO:
    Please refer to https://github.com/denisidoro/abra

EXAMPLES:
    # put "bar" into the "foo" channel
    echo "bar" | abra tx --channel foo               

    # put "bar" into the "foo" channel
    abra tx --channel foo --value "bar"   

    # print text that is put into the "foo" channel
    abra rx --channel foo                            

    # print the contents of filenames which are put into the "foo" channel
    abra rx --channel foo --cmd "cat {}"                        

    # force colorized output for ls
    abra faketty --cmd 'ls -la .'                    

    # run somecmd and pipe its stdout to the "foo_out" channel and its stderr to the "foo_err" channel
    somecmd > >(abra tx --channel foo_out) 2> >(abra tx --channel foo_err)"#)]
pub struct Opts {
    #[clap(subcommand)]
    pub cmd: Command,
}

#[derive(Clap)]
pub enum Command {
    /// Read from channel
    Rx(RxTxOpts),
    /// Write to channel
    Tx(RxTxOpts),
    /// Shell hooks for broadcasting the current directory
    Hook {
        #[clap(possible_values = &["bash", "zsh"], case_insensitive = true)]
        shell: Shell,
    },
    /// Force colorized outputs
    Faketty {
        #[clap(short, long)]
        cmd: String,
    },
}

#[derive(Clap, Debug)]
pub struct RxTxOpts {
    #[clap(short, long)]
    pub channel: String,
    #[clap(short, long)]
    pub value: Option<String>,
    #[clap(long)]
    pub cmd: Option<String>,
    #[clap(long)]
    pub clear: bool,
}

pub fn opts_from_env() -> Opts {
    Opts::parse()
}

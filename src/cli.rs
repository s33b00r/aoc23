use std::borrow::Cow;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(skip)]
    pub day: u8,

    /// Should the second part be solved? (default is to solve first)
    #[arg(short, long)]
    pub second: bool,

    /// Run in example mode (uses scratchpad input data)
    #[arg(short, long)]
    pub example: bool,

    #[cfg(target_pointer_width = "64")]
    #[arg(skip)]
    pub input: Cow<'static, str>,
    #[cfg(not(target_pointer_width = "64"))]
    #[arg(skip)]
    pub input: beef::Cow<'static, str>,

    #[arg(skip)]
    pub expected: Option<[String; 2]>,
}

pub(crate) fn args() -> Args {
    Args::parse()
}

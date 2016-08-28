use std::env;
use self_update::{self, InstallOpts};
use errors::*;
use clap::{App, Arg, AppSettings};
use rustup_dist::dist::TargetTriple;
use common;

pub fn main() -> Result<()> {
    let no_prompt = false;
    let verbose = false;
    let opts = InstallOpts {
        default_host_triple: "x86_64-unknown-linux-gnu".to_string(),
        default_toolchain: "stable-x86_64-unknown-linux-gnu".to_string(),
        no_modify_path: false,
    };

    try!(self_update::install(no_prompt, verbose, opts));

    Ok(())
}

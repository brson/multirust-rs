#![recursion_limit = "1024"]

extern crate rustup_dist;
#[macro_use]
extern crate rustup_utils;
#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate clap;
extern crate regex;
#[macro_use]
extern crate rustup;
extern crate term;
extern crate itertools;
extern crate time;
extern crate rand;
extern crate scopeguard;
extern crate tempdir;
extern crate sha2;
extern crate markdown;
extern crate libc;

#[macro_use]
mod log;
mod download_tracker;
mod self_update;
mod job;
mod term2;
mod errors;
mod help;

use std::env;
use std::path::PathBuf;
use errors::*;
use rustup_dist::dist::TargetTriple;

fn main() {
    if let Err(ref e) = run_multirust() {
        std::process::exit(1);
    }
}

fn run_multirust() -> Result<()> {
    match Some("blah") {
        Some(n) => {
            // NB: The above check is only for the prefix of the file
            // name. Browsers rename duplicates to
            // e.g. multirust-setup(2), and this allows all variations
            // to work.
            setup_mode_main()
        }
        _ => panic!()
    }
}

fn setup_mode_main() -> Result<()> {
    use self_update::{self, InstallOpts};
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

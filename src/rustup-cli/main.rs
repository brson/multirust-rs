#![allow(dead_code)]
#![allow(unused_imports)]
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

#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate winreg;
#[cfg(windows)]
extern crate user32;
#[cfg(windows)]
extern crate kernel32;
extern crate libc;

#[macro_use]
mod log;
mod common;
mod download_tracker;
mod setup_mode;
mod rustup_mode;
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
        common::report_error(e);
	panic!();
    }
}

fn run_multirust() -> Result<()> {
    let name = Some("rustup-init");
    match name {
        Some("rustup") => {
            rustup_mode::main()
        }
        Some(n) => {
            setup_mode::main()
        }
	_ => panic!()
    }
}

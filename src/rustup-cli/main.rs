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
mod proxy_mode;
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
        std::process::exit(1);
    }
}

fn run_multirust() -> Result<()> {
    match Some("blah") {
        Some("rustup") => {
            rustup_mode::main()
        }
        Some(n) => {
            // NB: The above check is only for the prefix of the file
            // name. Browsers rename duplicates to
            // e.g. multirust-setup(2), and this allows all variations
            // to work.
            setup_mode::main()
        }
        _ => panic!()
    }
}

fn do_compatibility_hacks() {
    make_environment_compatible();
    fix_windows_reg_key();
    delete_multirust_bin();
}

// Convert any MULTIRUST_ env vars to RUSTUP_ and warn about them
fn make_environment_compatible() {
    let ref vars = ["HOME", "TOOLCHAIN", "DIST_ROOT", "UPDATE_ROOT", "GPG_KEY"];
    for var in vars {
        let ref mvar = format!("MULTIRUST_{}", var);
        let ref rvar = format!("RUSTUP_{}", var);
        let mval = env::var_os(mvar);
        let rval = env::var_os(rvar);

        match (mval, rval) {
            (Some(mval), None) => {
                env::set_var(rvar, mval);
                warn!("environment variable {} is deprecated. Use {}.", mvar, rvar);
            }
            _ => ()
        }
    }
}

// #261 We previously incorrectly set HKCU/Environment/PATH to a
// REG_SZ type, when it should be REG_EXPAND_SZ. Silently fix it.
#[cfg(windows)]
fn fix_windows_reg_key() {
    use winreg::RegKey;
    use winreg::enums::RegType;
    use winapi::*;

    let root = RegKey::predef(HKEY_CURRENT_USER);
    let env = root.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE);

    let env = if let Ok(e) = env { e } else { return };

    let path = env.get_raw_value("PATH");

    let mut path = if let Ok(p) = path { p } else { return };

    if path.vtype == RegType::REG_EXPAND_SZ { return }

    path.vtype = RegType::REG_EXPAND_SZ;

    let _ = env.set_raw_value("PATH", &path);
}

#[cfg(not(windows))]
fn fix_windows_reg_key() { }

// rustup used to be called 'multirust'. This deletes the old bin.
fn delete_multirust_bin() {
    use rustup_utils::utils;
    use std::env::consts::EXE_SUFFIX;
    use std::fs;

    if let Ok(home) = utils::cargo_home() {
        let legacy_bin = home.join(format!("bin/multirust{}", EXE_SUFFIX));
        if legacy_bin.exists() {
            let _ = fs::remove_file(legacy_bin);
        }
    }
}

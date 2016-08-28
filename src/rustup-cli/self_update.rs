//! Self-installation and updating
//!
//! This is the installer at the heart of Rust. If it breaks
//! everything breaks. It is conceptually very simple, as rustup is
//! distributed as a single binary, and installation mostly requires
//! copying it into place. There are some tricky bits though, mostly
//! because of workarounds to self-delete an exe on Windows.
//!
//! During install (as `rustup-init`):
//!
//! * copy the self exe to $CARGO_HOME/bin
//! * hardlink rustc, etc to *that*
//! * update the PATH in a system-specific way
//! * run the equivalent of `rustup default stable`
//!
//! During upgrade (`rustup self upgrade`):
//!
//! * download rustup-init to $CARGO_HOME/bin/rustup-init
//! * run rustu-init with appropriate flags to indicate
//!   this is a self-upgrade
//! * rustup-init copies bins and hardlinks into place. On windows
//!   this happens *after* the upgrade command exits successfully.
//!
//! During uninstall (`rustup self uninstall`):
//!
//! * Delete `$RUSTUP_HOME`.
//! * Delete everything in `$CARGO_HOME`, including
//!   the rustup binary and its hardlinks
//!
//! Deleting the running binary during uninstall is tricky
//! and racy on Windows.

use errors::*;
use rustup_dist::dist;
use rustup_utils::utils;
use sha2::{Sha256, Digest};
use std::env;
use std::env::consts::EXE_SUFFIX;
use std::path::{Path, PathBuf};
use std::process::{self, Command};
use std::fs::{self, File};
use std::io::Read;
use tempdir::TempDir;
use regex::Regex;

pub struct InstallOpts {
    pub default_host_triple: String,
    pub default_toolchain: String,
    pub no_modify_path: bool,
}

/// Installing is a simple matter of coping the running binary to
/// CARGO_HOME/bin, hardlinking the various Rust tools to it,
/// and and adding CARGO_HOME/bin to PATH.
pub fn install(no_prompt: bool, verbose: bool,
               mut opts: InstallOpts) -> Result<()> {

    try!(do_pre_install_sanity_checks());
    try!(do_anti_sudo_check(no_prompt));

    Ok(())
}

fn do_pre_install_sanity_checks() -> Result<()> {

    let multirust_manifest_path
        = PathBuf::from("/usr/local/lib/rustlib/manifest-multirust");
    let rustc_manifest_path
        = PathBuf::from("/usr/local/lib/rustlib/manifest-rustc");
    let uninstaller_path
        = PathBuf::from("/usr/local/lib/rustlib/uninstall.sh");
    let multirust_meta_path
        = env::home_dir().map(|d| d.join(".multirust"));
    let multirust_version_path
        = multirust_meta_path.as_ref().map(|p| p.join("version"));
    let rustup_sh_path
        = env::home_dir().map(|d| d.join(".rustup"));
    let rustup_sh_version_path = rustup_sh_path.as_ref().map(|p| p.join("rustup-version"));

    let multirust_exists =
        multirust_manifest_path.exists() && uninstaller_path.exists();
    let rustc_exists =
        rustc_manifest_path.exists() && uninstaller_path.exists();
    let rustup_sh_exists =
        rustup_sh_version_path.map(|p| p.exists()) == Some(true);
    let old_multirust_meta_exists = if let Some(ref multirust_version_path) = multirust_version_path {
        multirust_version_path.exists() && {
            let version = utils::read_file("old-multirust", &multirust_version_path);
            let version = version.unwrap_or(String::new());
            let version = version.parse().unwrap_or(0);
            let cutoff_version = 12; // First rustup version

            version < cutoff_version
        }
    } else {
        false
    };

    match (multirust_exists, old_multirust_meta_exists) {
        (true, false) => {
            return Err("cannot install while multirust is installed".into());
        }
        (false, true) => {
            return Err("cannot install while multirust is installed".into());
        }
        (true, true) => {
            return Err("cannot install while multirust is installed".into());
        }
        (false, false) => ()
    }

    if rustc_exists {
        return Err("cannot install while Rust is installed".into());
    }

    if rustup_sh_exists {
        return Err("cannot install while rustup.sh is installed".into());
    }

    Ok(())
}

// If the user is trying to install with sudo, on some systems this will
// result in writing root-owned files to the user's home directory, because
// sudo is configured not to change $HOME. Don't let that bogosity happen.
fn do_anti_sudo_check(no_prompt: bool) -> Result<()> {
    #[cfg(unix)]
    #[inline(never)] // FIXME #679. Mysterious crashes on OS X 10.10+
    pub fn home_mismatch() -> bool {
        extern crate libc as c;

        use std::env;
        use std::ffi::CStr;
        use std::mem;
        use std::ops::Deref;
        use std::ptr;
        use std::os::raw::c_char;

        // test runner should set this, nothing else
        if env::var("RUSTUP_INIT_SKIP_SUDO_CHECK").as_ref().map(Deref::deref).ok() == Some("yes") {
            return false;
        }
        let mut buf = [0u8 as c_char; 1024];
        let mut pwd = unsafe { mem::uninitialized::<c::passwd>() };
        let mut pwdp: *mut c::passwd = ptr::null_mut();
        let len = buf.len();
        //let rv = unsafe { c::getpwuid_r(c::geteuid(), &mut pwd, buf.as_mut_ptr(), len, &mut pwdp) };
        /*if rv != 0 || pwdp == ptr::null_mut() {
            warn!("getpwuid_r: couldn't get user data");
            return false;
        }*/
        //let pw_dir = unsafe { CStr::from_ptr(pwd.pw_dir) }.to_str().ok();
        let pw_dir = Some("");
        let env_home = env::var_os("HOME");
        let env_home = env_home.as_ref().map(Deref::deref);
        match (env_home, pw_dir) {
            (None, _) | (_, None) => false,
            (Some(ref eh), Some(ref pd)) => eh != pd
        }
    }

    #[cfg(not(unix))]
    pub fn home_mismatch() -> bool {
        false
    }

    match (home_mismatch(), no_prompt) {
        (false, _) => (),
        (true, false) => {
            process::exit(1);
        },
        (true, true) => {
        }
    }

    Ok(())
}

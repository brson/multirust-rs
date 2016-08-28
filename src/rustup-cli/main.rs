extern crate libc;

use std::env;
use std::env::consts::EXE_SUFFIX;
use std::path::{Path, PathBuf};
use std::process::{self, Command};
use std::fs::{self, File};
use std::io::Read;

fn main() {
    let _ = setup_mode_main();
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
    let no_prompt = false;
    let verbose = false;
    let opts = InstallOpts {
        default_host_triple: "x86_64-unknown-linux-gnu".to_string(),
        default_toolchain: "stable-x86_64-unknown-linux-gnu".to_string(),
        no_modify_path: false,
    };

    try!(install(no_prompt, verbose, opts));

    Ok(())
}

struct InstallOpts {
    pub default_host_triple: String,
    pub default_toolchain: String,
    pub no_modify_path: bool,
}

/// Installing is a simple matter of coping the running binary to
/// CARGO_HOME/bin, hardlinking the various Rust tools to it,
/// and and adding CARGO_HOME/bin to PATH.
fn install(no_prompt: bool, verbose: bool,
           mut opts: InstallOpts) -> Result<()> {

    try!(do_anti_sudo_check(no_prompt));

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

type Result<T> = ::std::result::Result<T, Error>;
struct Error;

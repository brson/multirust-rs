extern crate libc;

use std::env;
use std::env::consts::EXE_SUFFIX;
use std::path::{Path, PathBuf};
use std::process::{self, Command};
use std::fs::{self, File};
use std::io::Read;

fn main() {
    let opts = InstallOpts {
        default_host_triple: "x86_64-unknown-linux-gnu".to_string(),
        default_toolchain: "stable-x86_64-unknown-linux-gnu".to_string(),
        no_modify_path: false,
    };

    let _ = install(opts);
}

struct InstallOpts {
    pub default_host_triple: String,
    pub default_toolchain: String,
    pub no_modify_path: bool,
}

fn install(mut opts: InstallOpts) -> Result<()> {

    try!(do_anti_sudo_check(false));

    Ok(())
}

#[inline(never)]
fn do_anti_sudo_check(no_prompt: bool) -> Result<()> {
    extern crate libc as c;

    use std::env;
    use std::ffi::CStr;
    use std::mem;
    use std::ops::Deref;
    use std::ptr;
    use std::os::raw::c_char;

    if env::var("RUSTUP_INIT_SKIP_SUDO_CHECK")
        .as_ref().map(Deref::deref).ok() == Some("yes") {
            return process::exit(1);
        }
    let mut buf = [0u8 as c_char; 1024];
    let mut pwd = unsafe { mem::uninitialized::<c::passwd>() };
    let mut pwdp: *mut c::passwd = ptr::null_mut();
    let len = buf.len();
    let pw_dir = Some("");
    let env_home = env::var_os("HOME");
    let env_home = env_home.as_ref().map(Deref::deref);
    let mismatch = match (env_home, pw_dir) {
        (None, _) | (_, None) => false,
        (Some(ref eh), Some(ref pd)) => eh != pd
    };

    match (mismatch, no_prompt) {
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

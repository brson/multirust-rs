use std::env;
use std::env::consts::EXE_SUFFIX;
use std::path::{Path, PathBuf};
use std::process::{self, Command};
use std::fs::{self, File};
use std::io::Read;

fn main() {
    let opts = InstallOpts;

    let _ = install(opts);
}

struct InstallOpts;

#[inline(never)]
fn install(mut opts: InstallOpts) -> Result<()> {
    use std::env;
    use std::ffi::CStr;
    use std::mem;
    use std::ops::Deref;
    use std::ptr;

    if env::var("RUSTUP_INIT_SKIP_SUDO_CHECK")
        .as_ref().map(Deref::deref).ok() == Some("yes") {
            panic!()
        }
    let mut buf = [0i8; 1024];
    let mut pwd = unsafe { mem::uninitialized::<passwd>() };
    let mut pwdp: *mut passwd = ptr::null_mut();
    let len = buf.len();
    let pw_dir = Some("");
    let env_home = env::var_os("HOME");
    let env_home = env_home.as_ref().map(Deref::deref);
    let mismatch = match (env_home, pw_dir) {
        (None, _) | (_, None) => false,
        (Some(ref eh), Some(ref pd)) => eh != pd
    };

    match (mismatch, false) {
        (false, _) => (),
        (true, false) => {
            process::exit(1);
        },
        (true, true) => {
        }
    }

    Ok(())
}

struct passwd {
    pub pw_name: *mut i8,
    pub pw_passwd: *mut i8,
    pub pw_uid: u32,
    pub pw_gid: u32,
    pub pw_gecos: *mut i8,
    pub pw_dir: *mut i8,
    pub pw_shell: *mut i8,
}

type Result<T> = ::std::result::Result<T, Error>;
struct Error;

//! Checks the internal consistency of rustup's metadata, attempting
//! to fix it where possible. Checks the envorinment for weirdness and
//! returns warnings about it.

use config::Cfg;
use errors::{Error, Result};
use multirust_dist::dist::PartialToolchainDesc;
use multirust_utils::utils;

/// Returns a vec of (problem, solution)
pub fn run(cfg: &Cfg) -> Result<Vec<(String, String)>> {
    let mut notices = Vec::new();
    notices.extend(check_old_toolchain_names(cfg));
    //notices.extend(check_multirust_rs_bins());
    //notices.extend(check_multirust_sh_install());
    //notices.extend(check_rust_standalone_install());
    //notices.extend(check_rust_unix_install());

    notices
}

// Around 2016/03/31 rustup began using target-qualified toolchain
// names for storage, leaving the `toolchains/nightly`
// etc. directories
fn check_old_toolchain_names(cfg: &Cfg) -> Result<Vec<(String, String)>> {
    let mut notices = Vec::new();

    for dir in try!(utils::read_dir("toolchains", &cfg.toolchains_dir)) {
        let dir = try!(dir.map_err(|e| Error::SelfCheckIoError(e)));
        let name = dir.path().file_stem();

        if name.is_none() { continue };
        let name = name.expect("");

        let toolchain = PartialToolchainDesc::from_str(name);
        if toolchain.is_err() {
            // custom toolchain
            continue
        }
        let toolchain = toolchain.expect("");

        if toolchain.arch.is_none() || toolchain.os.is_none() {

            let prob = format!(
                "the toolchain directory at {} uses an old naming scheme",
                dir.path().display());
            let soln = format!("delete it");

            notices.push((prob, soln));
        }
    }

    notices
}

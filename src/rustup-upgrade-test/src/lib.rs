extern crate tempdir;

use std::env;
use std::path::PathBuf;
use std::process::Command;
use tempdir::TempDir;

pub static KNOWN_REVS: &'static [&'static str] = &[
    "0.1.10",
    "0.1.11",
    "0.1.12",
    "0.2.0"
];

pub fn run_test() {
    let host_triple = env::var("RUSTUP_HOST").expect("set RUSTUP_HOST");
    let archive_server = env::var("RUSTUP_ARCHIVE").expect("set RUSTUP_ARCHIVE");
    let dl_dir = TempDir::new("rustup-test").expect("tempdir");
    let dl_dir = dl_dir.path();
    let home_dir = TempDir::new("rustup-test").expect("tempdir");
    let home_dir = home_dir.path();
    let cargo_home = home_dir.join(".cargo");
    let rustup_home = home_dir.join(".rustup");

    println!("archive server: {}", archive_server);
    println!("dl dir: {}", dl_dir.display());

    for (i, rev) in KNOWN_REVS.iter().enumerate() {
        let update_root = format!("{}/{}", archive_server, rev);
        println!("update root: {}", update_root);

        let rustup_init = format!("rustup-init{}", env::consts::EXE_SUFFIX);

        if i == 0 {
            let archive_init = format!("{}/{}/{}",
                                       update_root,
                                       host_triple,
                                       rustup_init);
            let local_init = format!("{}/{}", dl_dir.display(), rustup_init);
            let mut cmd = Command::new("curl");
            cmd
                .arg("-sSLf")
                .arg(&archive_init)
                .arg("-o")
                .arg(&local_init)
                .status()
                .expect("curl");

            let mut cmd = Command::new(&local_init);
            cmd
                .arg("-y")
                .env("CARGO_HOME", &cargo_home)
                .env("RUSTUP_HOME", &rustup_home)
                .env("MULTIRUST_HOME", &rustup_home)
                .status()
                .expect("rustup-init");
        } else {
            panic!()
        }
    }
}

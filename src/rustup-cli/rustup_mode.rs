use clap::{App, Arg, ArgGroup, AppSettings, SubCommand, ArgMatches};
use common;
use rustup::{Cfg, Toolchain, command};
use rustup::settings::TelemetryMode;
use errors::*;
use rustup_dist::manifest::Component;
use rustup_dist::dist::{TargetTriple, PartialToolchainDesc, PartialTargetTriple};
use rustup_utils::utils;
use self_update;
use std::path::Path;
use std::process::Command;
use std::iter;
use term2;
use std::io::Write;
use help::*;

pub fn main() -> Result<()> {
    let ref matches = cli().get_matches();
    let ref cfg = try!(common::set_globals(false));

    match matches.subcommand() {
        ("list", Some(_)) => try!(common::list_overrides(cfg)),
        ("set", Some(m)) => try!(override_add(cfg, m)),
        (_ ,_) => unreachable!(),
    }

    Ok(())
}

pub fn cli() -> App<'static, 'static> {
    App::new("rustup")
}

fn override_add(cfg: &Cfg, m: &ArgMatches) -> Result<()> {
    let ref toolchain = m.value_of("toolchain").expect("");
    let toolchain = try!(cfg.get_toolchain(toolchain, false));

    let status = if !toolchain.is_custom() {
        Some(try!(toolchain.install_from_dist_if_not_installed()))
    } else if !toolchain.exists() {
        return Err(ErrorKind::ToolchainNotInstalled(toolchain.name().to_string()).into());
    } else {
        None
    };

    try!(toolchain.make_override(&try!(utils::current_dir())));

    if let Some(status) = status {
        println!("");
        try!(common::show_channel_update(cfg, toolchain.name(), Ok(status)));
    }

    Ok(())
}


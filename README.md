# rustup - The Rust toolchain installer

`rustup` installs Rust from the official release channels, enabling
you to easily switch between stable, beta, and nightly compilers
and keep them updated.

If you are programming in Rust you should be using `rustup`.

**WARNING: Don't use rustup yet! This is seriously beta software.**

## Installation

Follow the instructions at https://www.rustup.rs.

After installation you will have executables `rustc`, `cargo`, and
`rustup` in *Cargo's 'bin' directory*, which on Unix is located at
`$HOME/.cargo/bin` and on Windows is `%USERPROFILE%\.cargo\bin`. In
addition to the compiler this directory will contain any tools you
install with `cargo install`.

These directories will be in your `$PATH` environment variable, which
means you can run them from the shell without further configuration.

Open a *new* shell and type the following:

```
rustc --version
```

If you see something like `rustc 1.7.0 (a5d1e7a59 2016-02-29)` then
you are ready to Rust.

## Keeping Rust up to date

Rust is distributed on three different [release channels]: stable,
beta, and nightly. `rustup` is configured to use the stable channel by
default, which represents represents the latest release of Rust,
and is released every six weeks.

When a new version of Rust is released, you can type `rustup` to update
to it:

```
$ rustup
info: updating existing install for 'stable'
info: downloading toolchain manifest
info: downloading component 'rustc'
info: downloading component 'rust-std'
info: downloading component 'rust-docs'
info: downloading component 'cargo'
info: installing component 'rustc'
info: installing component 'rust-std'
info: installing component 'rust-docs'
info: installing component 'cargo'
info: toolchain 'stable' installed

stable updated:

rustc 1.7.0 (a5d1e7a59 2016-02-29)
cargo 0.8.0-nightly (28a0cbb 2016-01-17)
```

This is the essense of `rustup`.

[release channels]: https://github.com/rust-lang/rfcs/blob/master/text/0507-release-channels.md

## Working with nightly Rust

Rustup gives you easy access to the nightly compiler. To add it just run `rustup update nightly`:

```
$ rustup update nightly
info: installing toolchain 'nightly'
info: downloading toolchain manifest
info: downloading component 'rustc'
info: downloading component 'rust-std'
info: downloading component 'rust-docs'
info: downloading component 'cargo'
info: installing component 'rustc'
info: installing component 'rust-std'
info: installing component 'rust-docs'
info: installing component 'cargo'
info: toolchain 'nightly' installed

nightly updated:

rustc 1.9.0-nightly (02310fd31 2016-03-19)
cargo 0.10.0-nightly (132b82d 2016-03-19)
```

Now Rust nightly is installed, but not activated. To test it out you
can run a command from the nightly toolchain like

```
$ rustup run rustc --version
rustc 1.9.0-nightly (02310fd31 2016-03-19)
```

But more likely you want to use it for a while. To switch to nightly
globally, change the default with `rustup default nightly`:

```
$ rustup default nightly
info: using existing install for 'nightly'
info: default toolchain set to 'nightly'

nightly revision:

rustc 1.9.0-nightly (02310fd31 2016-03-19)
cargo 0.10.0-nightly (132b82d 2016-03-19)
```

Now any time you run `cargo` or `rustc` you will be running the
nightly compiler.

With nightly installed any time you run `rustup`, the nightly channel
will be updated:

```
$ rustup
info: updating existing install for 'stable'
info: downloading toolchain manifest
info: downloading component 'rust'
info: toolchain is already up to date
info: updating existing install for 'nightly'
info: downloading toolchain manifest
info: downloading component 'rust'
info: toolchain is already up to date

stable unchanged:

rustc 1.7.0 (a5d1e7a59 2016-02-29)
cargo 0.8.0-nightly (28a0cbb 2016-01-17)

nightly unchanged:

rustc 1.9.0-nightly (02310fd31 2016-03-19)
cargo 0.10.0-nightly (132b82d 2016-03-19)
```

## Cross-compilation

## Overrides

## Toolchain specification

## Uninstallation

## How rustup works

## Examples

- Set the default toolchain to the latest nightly:
  `rustup default nightly`

- List all targets:
  `rustup target list`

- Install the Android target:
  `rustup target add arm-linux-androideabi`

- Remove the Android target:
  `rustup target remove arm-linux-androideabi`

- For the current directory, use the most recent stable build using the MSVC linker:
  `rustup override stable-msvc`

- For the current directory, use a 32-bit beta build instead:
  `rustup override beta-i686`

- For the current directory, use a nightly from a specific date:
  `rustup override nightly-2015-04-01`

- Combine these:
  `rustup override msvc-nightly-2015-04-01-i686`

- Install a custom toolchain by symlinking an existing installation:
  `rustup toolchain link my-toolchain "C:\RustInstallation"`

- Switch back to the default toolchain for the current directory:
  `rustup override remove`

- See which toolchain will be used in the current directory:
  `rustup override show`

## Environment variables

- `RUSTUP_TOOLCHAIN` (default: none)
  If set, will override the toolchain used for all rust tool
  nvocations. A toolchain with this name should be installed, or
  invocations will fail.

- `RUSTUP_DIST_ROOT` (default: `https://static.rust-lang.org/dist`)
  Sets the root URL for downloading packages. You can change this to
  instead use a local mirror, or to test the binaries from the staging
  directory.

- `RUSTUP_HOME` (default: `~/.rustup` or `%USERPROFILE%/.rustup`)
  Sets the root multirust folder, used for storing installed
  toolchains and configuration options.

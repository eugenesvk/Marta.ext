#![allow(unused_imports,unused_variables,unreachable_code,dead_code,non_upper_case_globals)]
//! This binary defines various auxiliary build commands that `cargo` can't handle, e.g.:
//! - `cargo test -p xtask` for special testing
//! - `cargo xtask docgen` for for code generation
//! - `cargo xtask query-check` for query verification
//!
//! It's integrated into the `cargo` command line by using an alias in `.cargo/config`<br>
//! See [cargo-xtask](https://github.com/matklad/cargo-xtask)

extern crate helperes      as h    ;
extern crate helperes_proc as hproc;
use ::h            	::*; // gets macros :: prefix needed due to proc macro expansion
pub use hproc      	::*; // gets proc macros
pub use ::h::alias 	::*;
pub use ::h::helper	::*;

use anyhow::{Result,Context,bail};
use std  	::{env,fs,
  path   	::{Path,PathBuf},
  process	::{Command,Stdio},
};
// use xshell::{cmd, Shell};

fn main() -> anyhow::Result<()> {
  try_main()
}
use dummy_lib::*;
fn try_main() -> anyhow::Result<()> {
  let task = env::args().nth(1);
  p!("task_arg1 = {:?} lib = {:?}", task, dummy_lib::lib());
  match task {
    None           	=> tasks::print_help(),
    Some(t)        	=> match t.as_str() {
      "docgen"     	=> tasks::docgen()?,
      "query-check"	=> tasks::querycheck()?,
      invalid      	=> return bail!("Invalid task name: {}", invalid),
    },
  };
  Ok(())
}


pub mod tasks {
  use crate::*;

  pub fn docgen    () -> Result<()> {Ok(())}
  pub fn querycheck() -> Result<()> {Ok(())}

  pub fn print_help() {
    use indoc::formatdoc;
    let help_out = formatdoc!("
      Use: ‘cargo x <task>’, where <task> is one of ↓
        docgen     \tGenerate files to be included somewhere
        query-check\tCheck the validity of some queries"
    );
    pe!("{}", help_out);
  }
}

fn project_root() -> PathBuf {Path::new(&env!("CARGO_MANIFEST_DIR")).ancestors().nth(1).unwrap().into()}
fn dist_dir    () -> PathBuf {project_root().join("target/dist")}

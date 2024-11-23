#![allow(unused_imports,unused_variables,unreachable_code,dead_code,non_upper_case_globals)]
extern crate helperes      as h    ;
extern crate helperes_proc as hproc;
use ::h            	::*; // gets macros :: prefix needed due to proc macro expansion
pub use hproc      	::*; // gets proc macros
pub use ::h::alias 	::*;
pub use ::h::helper	::*;

_mod!(binmod); //→ #[path="binmod/[binmod].rs"] pub mod binmod;
use crate::binmod::print42;

use std::error::Error;
use std::result;

use std     	::{//env,fs,
  path      	::{Path,PathBuf},
  // process	::{Command,Stdio},
};
type Result<T> = result::Result<T, Box<dyn Error>>;

static _dbg:i8 = 1;
/// Quick and dirty way to disable blocks of debug-level code, use `if _d(1) {}` to do something only if global _dbg <= 1
pub fn _d(lvl:i8) -> bool {if lvl>=_dbg{true}else{false}}

fn main() -> Result<()> {
  print42()?;

  use clipboard_files;
  match clipboard_files::read() { //:Vec<std::path::PathBuf>
    Ok (paths)	=> {p!("got №{} paths: {:?}",paths.len(),paths)?; p!("{}", type_of(paths))?;},
    Err(e)    	=> {p!("not files")?;},
  }
  if _d(0) {p!("debug is on at level1")?;}
  Ok(())
}

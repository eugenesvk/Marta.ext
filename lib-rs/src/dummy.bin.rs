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
fn main() -> Result<()> {
  print42()?;

  use clipboard_files;
  match clipboard_files::read() { //:Vec<std::path::PathBuf>
    Ok (paths)	=> {p!("got №{} paths: {:?}",paths.len(),paths)?; p!("{}", type_of(paths))?;},
    Err(e)    	=> {p!("not files")?;},
  }
  Ok(())
}

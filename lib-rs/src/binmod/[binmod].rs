extern crate helperes      as h    ;
extern crate helperes_proc as hproc;
use ::h            	::*; // gets macros :: prefix needed due to proc macro expansion
pub use hproc      	::*; // gets proc macros
pub use ::h::alias 	::*;
pub use ::h::helper	::*;
// use crate::*;

use std::error::Error;
use std::result;

type Result<T> = result::Result<T, Box<dyn Error>>;
pub fn print42() -> Result<()> {p!("{}",42)?; Ok(())}

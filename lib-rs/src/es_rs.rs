#![allow(unused_imports,unused_variables,unreachable_code,dead_code,non_upper_case_globals)]
extern crate helperes      as h    ;
extern crate helperes_proc as hproc;
use ::h            	::*; // gets macros :: prefix needed due to proc macro expansion
pub use hproc      	::*; // gets proc macros
pub use ::h::alias 	::*;
pub use ::h::helper	::*;

#[path="libmod/[libmod].rs"] pub mod libmod;
use crate::libmod::{sum,used_memory,hello,console,setup_logging};

use mlua::prelude::*;

// check ./.cargo/config.toml for macOS specific compiler flags
#[mlua::lua_module(name="libes_rs")] //register Lua module entrypoint, MUST match require(module_file_name)
fn my_lua_lib(lua:&Lua) -> LuaResult<LuaTable> { // exported as _luaopen_libes_rs, see `nm -gU libes_rs.rs`
  setup_logging()?;
  let exports = lua.create_table()?;
  exports.set("sum"        , lua.create_function(sum        )?)?;
  exports.set("used_memory", lua.create_function(used_memory)?)?;
  exports.set("console"    , lua.create_function(console    )?)?;
  Ok(exports)
}

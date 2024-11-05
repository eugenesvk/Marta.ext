use mlua::prelude::*;
pub fn sum        (_  :&Lua, (a,b):(i64,i64)) -> LuaResult<i64  > {Ok(a + b)            }
pub fn used_memory(lua:&Lua, _    :(       )) -> LuaResult<usize> {Ok(lua.used_memory())}

use tracing::{info,warn,Level};
use tracing_subscriber::prelude::*; // added error check
use tracing_oslog::OsLogger;
const log_subsystem:&'static str = "Marta.es_rs";
const log_category :&'static str = "default";
pub fn setup_logging() -> LuaResult<()> {
  let collector = tracing_subscriber::registry().with(OsLogger::new(log_subsystem,log_category));
  tracing::subscriber::set_global_default(collector).expect("failed to set global subscriber"); //⚠️ libs should avoid this to not cause conflicts when executables that depend on the library try to set the default later
  Ok(())
}

pub fn console(lua:&Lua,in_str:String) -> LuaResult<i8> {
  warn!(myvar="⚠️","fn console(in_str)@Marta's Rust lua module es_rs.rs, in_str=‘{:?}’", in_str); // this creates a new event, outside of any spans.
  Ok(0)
}

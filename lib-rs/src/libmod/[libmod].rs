use crate::*;

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

pub fn ret_s(lua:&Lua, _:()) -> LuaResult<LuaString> { // example of a String being returned back to lua
  let s_rs:String="fn ret_Str()@Marta's Rust lua module".to_string();
  // let s_rs:String=type_of(&lua).to_string();
  // let s:LuaString = lua.create_string("libmod.rs").unwrap();
  let s_lua:LuaString = lua.create_string(s_rs)?;
  Ok(s_lua)
}

use anyhow::{anyhow, Result};
use mlua::Function;
pub fn call_lua_fn(lua:&Lua, _:()) -> LuaResult<LuaString> { // example of using `mytostring = tostring`
  let g = lua.globals();
  let mytostring:Function = g.get("mytostring")?;
  let s_rs:String = mytostring.call::<String>(123)?;
  let s_lua:LuaString = lua.create_string(s_rs)?;

  let s_lua1:LuaString = lua.create_string("glue.get(myalert).call::<()>(s_lua1)")?;
  //1ok
  // let myalert:Function = g.get("myalert")?;
  // let _ = myalert.call::<()>(s_lua1)?;
  //2
  // let got:LuaString = match g.get::<LuaValue>("myalert")? {
  //   LuaValue::Function(myalert)	=> myalert.call((s_lua1,)),
  //   _                          	=> Err(LuaError::RuntimeError("❗ globals.myalert wrong type ≠ 𝑓𝑛".into())),
  //   }?;
  //missing myalert1 shows no context in Console from Marta -- 'apply()' failed:LuaUserData(ptr:Optional(0x0000600001071cb8))
  //3 custom console error message works, but how to make LuaError show up? Is it blocked by Marta???
  let got = match g.get::<LuaValue>("myalert1") {
    Ok(val)                      	=> match val {
      LuaValue::Function(myalert)	=> myalert.call::<()>(s_lua1),
      _                          	=> {warn!("❗ globals.myalert1 not found"); Err(anyhow!("❗"))},
      // _                       	=> {warn!("❗ globals.myalert1 not found"); Err(LuaError::RuntimeError("❗".into()))},
      }                          	,
    _                            	=> {warn!("❗ globals.myalert1 wrong type ≠ 𝑓𝑛"); Err(LuaError::RuntimeError("❗".into()))},
    }?;

  let s_lua2:LuaString = lua.create_string("g.get(martax).get(alert).call::<()>(s_lua2)")?;
  let martax:LuaTable = g.get("martax")?;
  let alert:Function = martax.get("alert")?;
  let _ = alert.call::<()>(s_lua2)?;

  let get_application_url:Function = martax.get("getApplicationUrl")?;
  let s_app:LuaString = get_application_url.call::<LuaString>("com.apple.calculator")?;
  let _ = alert.call::<()>(s_app)?;

use std     	::{//env,fs,
  path      	::{Path,PathBuf},
  // process	::{Command,Stdio},
};
use clipboard_files;
pub fn cut     (lua:&Lua, _:()) -> LuaResult<LuaString> {
  let s_lua:LuaString = lua.create_string("stub for a cut")?;
  Ok(s_lua)
}
pub fn move    (lua:&Lua, _:()) -> LuaResult<LuaString> {
  let s_lua:LuaString = lua.create_string("stub for a move")?;
  Ok(s_lua)
}
pub fn move_any(lua:&Lua, _:()) -> LuaResult<LuaString> {
  let g = lua.globals();

  let mut out_s:String = "".to_string();
  match clipboard_files::read() { //:Vec<std::path::PathBuf>
    Ok (paths)	=> {out_s.push_str(format!("got №{}",paths.len()).as_ref());},
    Err(e)    	=> {out_s.push_str("clipboard has no files");},
  }
  let martax:LuaTable = g.get("martax")?;
  let alert:Function = martax.get("alert")?;
  let s_lua:LuaString = lua.create_string(out_s)?;
  // let _ = alert.call::<()>(s_lua)?;

  Ok(s_lua)
}

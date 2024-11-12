use crate::*;
#[path="gui/[gui].rs"] pub mod gui;
pub use gui	::*;
pub mod objc_helper;
pub use objc_helper::*;

use std     	::{//env,fs,
  path      	::{Path,PathBuf},
  // process	::{Command,Stdio},
  cell      	::{Cell,LazyCell},
};
use core::ffi::c_void;

use anyhow::{anyhow, Result};

use mlua::prelude::*;
pub fn used_memory(lua:&Lua, _    :(       )) -> LuaResult<usize> {Ok(lua.used_memory())}

use tracing::{info,warn,Level};
use tracing_subscriber::prelude::*; // added error check
use tracing_oslog::OsLogger;
const log_subsystem:&'static str = "Marta.es_rs";
const log_category :&'static str = "plugin";
pub fn setup_logging() -> LuaResult<()> {
  let collector = tracing_subscriber::registry().with(OsLogger::new(log_subsystem,log_category));
  tracing::subscriber::set_global_default(collector).expect("failed to set global subscriber"); //⚠️ libs should avoid this to not cause conflicts when executables that depend on the library try to set the default later
  Ok(())
}

use mlua::{Function,Variadic};
use clipboard_files;
pub fn cut     (lua:&Lua, _:()) -> LuaResult<LuaString> {
  let s_lua:LuaString = lua.create_string("stub for a cut")?;
  Ok(s_lua)
}

mod marta_api_const; // Associated constant with a struct for autocomplete and typo avoidance
pub use marta_api_const::*;

pub fn move_cb_to(lua:&Lua, (ctx_a, path_to):(LuaAnyUserData, PathBuf)) -> LuaResult<LuaString> { // move clipboard files to the destination
  let g      	:LuaTable 	= lua.globals();
  let marta  	:LuaTable 	= g.get("marta" 	)?;
  let martax 	:LuaTable 	= g.get("martax"	)?;
  let plug_id	:LuaString	= g.get("plugID"	)?;// requires global plugID in the init.lua
  let pss    	:Function 	= g.get("pss"   	)?;// ...
  let psl    	:Function 	= g.get("psl"   	)?;// ...

  let alert	:Function	= martax.get("alert" )?;

  let ctx_w  	:LuaAnyUserData	= ctx_a.get(ctxA::window      	)?;//
  let ctx_g  	:LuaAnyUserData	= marta.get("globalContext"   	)?;//
  let act_g  	:LuaAnyUserData	= ctx_g.get("actions"         	)?;//
  // let fs_l	:LuaAnyUserData	= marta.localFileSystem       	--
  let ctx_pa 	:LuaAnyUserData	= ctx_a.get(ctxA::activePane  	)?;//
  let ctx_pn 	:LuaAnyUserData	= ctx_a.get(ctxA::inactivePane	)?;//
  let model_a	:LuaAnyUserData	= ctx_pa.get(ctxP::model      	)?;// Active pane list model
  let view_p 	:LuaAnyUserData	= ctx_pa.get(ctxP::view       	)?;//

  // there is a check in lua, so this is just in case
  let is_fs:bool = model_a.get::<LuaValue>("isLocalFileSystem")?.as_boolean().expect("isLocalFileSystem should be a bool");
  if !is_fs {return Ok(lua.create_string("📋 can't run in a non-local filesystem")?)}
  if !path_to.is_dir() {let s_lua:LuaString = lua.create_string(format!("❗not a 📁, can't paste here"))?; let _ = pss.call::<()>(s_lua.clone()); return Ok(s_lua)}  // zip-fs report path as / in Marta, so this doesn't help there

  let mut cb_res = String::new();
  let cb_paths:Vec<std::path::PathBuf> = match clipboard_files::read() {
    Ok (paths)	=> paths,
    Err(e)    	=> {let s_lua:LuaString = lua.create_string(format!("📋clipboard has no dir/file items"))?; let _ = pss.call::<()>(s_lua.clone()); return Ok(s_lua)},
  };
  cb_res.push_str(format!("📋clipboard has №{} dir/file items",cb_paths.len()).as_ref());

  warn!("move_cb_to"); // this creates a new event, outside of any spans.
  // warn!("fn console(in_str)@Marta's Rust lua module es_rs.rs, in_str=‘{:?}’", path_to); // this creates a new event, outside of any spans.

  // let sss:mlua::Value = path_to.into_os_string().into_lua(lua)?;
  // let s_lua:LuaString = lua.create_string(sss)?;
  // let s_lua:LuaString = lua.create_string(path_to.into_lua(lua)?)?;
  // let s_lua:mlua::Value::String = path_to.into_lua(lua)?;
  // let _ = alert.call::<()>(s_lua.clone())?;

  let s_lua:LuaString = lua.create_string("stub for a cut")?;
  Ok(s_lua)
}

use crate::*;
#[path="gui/[gui].rs"] pub mod gui;
pub use gui	::*;
pub mod objc_helper;
pub use objc_helper::*;
pub mod fs_rem;
pub use fs_rem::*;

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
static _dbg:i8 = 1;
/// Quick and dirty way to disable blocks of debug-level code, use `if _d(1) {}` to do something only if global _dbg ≥ 1
pub fn _d(lvl:i8) -> bool {if _dbg>=lvl{true}else{false}}

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
  if !path_to.is_dir() {let s_lua:LuaString = lua.create_string(format!("❗not a 📁, can't paste here"))?; let _ = psl.call::<()>(s_lua.clone()); return Ok(s_lua)}  // zip-fs report path as / in Marta, so this doesn't help there

  let mut cc_res = String::new();
  let cb_paths:Vec<std::path::PathBuf> = match clipboard_files::read() {
    Ok (paths)	=> paths,
    Err(e)    	=> {let s_lua:LuaString = lua.create_string(format!("📋clipboard has no dir/file items"))?; let _ = psl.call::<()>(s_lua.clone()); return Ok(s_lua)},
  };
  cc_res.push_str(format!("📋clipboard has №{} dir/file items",cb_paths.len()).as_ref());

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

use cacao::foundation::{id, nil, to_bool, NSInteger, NSString, NSUInteger, AutoReleasePool};
use objc::ffi::{NO,YES};
use cacao::objc::runtime::Object;
// use objc::runtime::Object;
use objc::{class, msg_send, sel};
use objc_id::{ShareId,Id,Shared};

pub fn ask_overwrite(lua:&Lua, (ctx_a, path_to, args_force):(LuaAnyUserData, PathBuf, Option<LuaValue>)) -> LuaResult<LuaString> { // move clipboard files to the destination
  // ctx_a = forwarded marta.ActionContext passed to action's apply function(), marta.sh/api/marta/actioncontext.type
  // path_to = path string (marta.sh/api/marta/path.type/rawvalue) to move items to
  // args_force = bool, whether to force overwrite without asking user's confirmation
  // 0 Setup various helper constants
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

  // 1 Parse arguments passed from lua
  let nswin_lua 	:LuaLightUserData	= ctx_w.get(ctxW::nsWindow	)?;//LightUserData<NSWindow>, equivalent to an unmanaged raw pointer
  let nswin_rptr	:*mut c_void     	= nswin_lua.0; // get the pointer
  if nswin_rptr.is_null() {let s_lua:LuaString = lua.create_string("✗ nswin_ptr.is_null")?;let _ = alert.call::<()>(s_lua.clone())?;
    return Ok(lua.create_string("📋 got no pointer to the main window, can't create any dialogs…")?)
  }

  let force = match args_force {Some(LuaValue::Boolean(force_arg)) => force_arg,  _=>false};
  if _d(2) {let s_rs:String = format!("got args_force: {} and force={}",args_force.is_some(),force);
  let s_lua:LuaString = lua.create_string(&s_rs)?;let _ = alert.call::<()>(s_lua.clone())?;}

  // there is a check in lua, so this is just in case
  let is_fs:bool = model_a.get::<LuaValue>("isLocalFileSystem")?.as_boolean().expect("isLocalFileSystem should be a bool");
  if !is_fs {return Ok(lua.create_string("📋 can't run in a non-local filesystem")?)}
  if !path_to.is_dir() {let s_lua:LuaString = lua.create_string(format!("❗not a 📁, can't paste here"))?; let _ = psl.call::<()>(s_lua.clone()); return Ok(s_lua)}  // zip-fs report path as / in Marta, so this doesn't help there

  let mut cc_res = String::new();
  let cc_paths:Vec<std::path::PathBuf> = match clipboard_files::read() {
    Ok (paths)	=> paths,
    Err(e)    	=> {let s_lua:LuaString = lua.create_string(format!("📋clipboard has no dir/file items"))?; let _ = psl.call::<()>(s_lua.clone()); return Ok(s_lua)},
  };
  cc_res.push_str(format!("📋clipboard has №{} dir/file items",cc_paths.len()).as_ref());

  /*
  warn!("move_cb_to"); // this creates a new event, outside of any spans.
  // warn!("fn console(in_str)@Marta's Rust lua module es_rs.rs, in_str=‘{:?}’", path_to); // this creates a new event, outside of any spans.

  // let sss:mlua::Value = path_to.into_os_string().into_lua(lua)?;
  // let s_lua:LuaString = lua.create_string(sss)?;
  // let s_lua:LuaString = lua.create_string(path_to.into_lua(lua)?)?;
  // let s_lua:mlua::Value::String = path_to.into_lua(lua)?;
  // let _ = alert.call::<()>(s_lua.clone())?;
  */
  use std::{ffi::OsString, path::PathBuf};
  use uu_mv::{BackupMode, UpdateMode};
  // 🗇🗍🗊🗐 suffix doesn't work with numbered backup
  let overwrite_no                	= false;
  let overwrite_ask               	= false;
  let overwrite = if overwrite_no 	{uu_mv::OverwriteMode::NoClobber
  } else          if overwrite_ask	{uu_mv::OverwriteMode::Interactive
  } else                          	{uu_mv::OverwriteMode::Force};
  let progress_bar                	= true;
  let verbose                     	= false;
  let arg_update                  	= false;
  let update = if arg_update      	{UpdateMode::ReplaceIfOlder
  } else                          	{UpdateMode::ReplaceAll};
  let arg_backup = "no";
  let backup = match arg_backup {
    "no"	=> BackupMode::NoBackup,
    "s" 	=> BackupMode::SimpleBackup,
    "#" 	=> BackupMode::NumberedBackup,
    "x" 	=> BackupMode::ExistingBackup,
    _   	=> BackupMode::NoBackup,
  };
  let suffix = String::from("~");
  let target_dir = Some(path_to.into_os_string());
  //todo: how to do interactive intput from within a gui? redirect stdio?? or something

  let options = uu_mv::Options {overwrite,progress_bar,verbose,suffix,backup,update, target_dir,no_target_dir:false,
    strip_slashes:false, debug:false,};
  let cc_paths_oss = cc_paths.into_iter().map(|p| p.into_os_string()).collect::<Vec<OsString>>();

  if let Err(error) = uu_mv::mv(&cc_paths_oss, &options) {
    let s_lua:LuaString = lua.create_string(format!("❗mv error"))?; let _ = pss.call::<()>(s_lua.clone()); return Ok(s_lua)
    // return Err(ShellError::GenericError {
    //   error: format!("{}", error),
    //   msg: format!("{}", error),
    //   span: None,
    //   help: None,
    //   inner: Vec::new(),
    // });
  }

  // // Ask user confirmation for overwriting
  // let win_id_objc:ShareId<Object> = get_win_id_objc(&nswin_lua)?; // convert pointer to Objc type
  // WM.with(|wm| {
  //   wm.save_marta(win_id_objc.clone()); //todo: move saving marta to open modal (check if saved)
  //   wm.open_sheet(win_id_objc);
  //   wm.on_message(Message::OpenOverwriteSheet); //todo: replace ↑
  // });
  // //TODO ↑ this returns after showing, how to block?

  Ok(lua.create_string("Ok")?)
}
// todo: how to ask interactively about each item?



pub fn clipboard_trash(lua:&Lua, (ctx_a, is_confirm_a):(LuaAnyUserData, Option<LuaValue>)) -> LuaResult<LuaString> { // move clipboard files to trash, setting their extended attributes to the original path so that you could undo the operation later
  // ctx_a = forwarded marta.ActionContext passed to action's apply function(), marta.sh/api/marta/actioncontext.type
  // is_confirm_a = bool, whether to ask user's confirmation of the operation
  // 0 Setup various helper constants
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

  // 1 Parse arguments passed from lua
  let nswin_lua 	:LuaLightUserData	= ctx_w.get(ctxW::nsWindow	)?;//LightUserData<NSWindow>, equivalent to an unmanaged raw pointer
  let nswin_rptr	:*mut c_void     	= nswin_lua.0; // get the pointer
  if nswin_rptr.is_null() {let s_lua:LuaString = lua.create_string("✗ nswin_ptr.is_null")?;let _ = alert.call::<()>(s_lua.clone())?;
    return Ok(lua.create_string("📋 got no pointer to the main window, can't create any dialogs…")?)
  }

  let is_confirm = match is_confirm_a {Some(LuaValue::Boolean(is_confirm_val)) => is_confirm_val,  _=>true};
  if _d(1) {let s_rs:String = format!("got is_confirm_a: {} and is_confirm={}",is_confirm_a.is_some(),is_confirm);
  let s_lua:LuaString = lua.create_string(&s_rs)?;let _ = alert.call::<()>(s_lua.clone())?;}

  // 2 Check clipboard
  let mut cc_res = String::new();
  let cc_paths:Vec<std::path::PathBuf> = match clipboard_files::read() {
    Ok (paths)	=> paths,
    Err(e)    	=> {let s_lua:LuaString = lua.create_string(format!("⎋: 📋clipboard has no dir/file items"))?; let _ = psl.call::<()>(s_lua.clone()); return Ok(s_lua)},
  };
  cc_res.push_str(format!("📋clipboard has №{} dir/file items",cc_paths.len()).as_ref());
  // todo: save the number to show in the dialog message box

  // 3.1 Delete if no user confirmation is needed
  if !is_confirm {
    let _ = trash_all(&cc_paths);
    return Ok(lua.create_string("Deleted")?)
  }

  // 3.2 Setup a callback to create a list of paths once a user confirmation is given
  let s_lua:LuaString = lua.create_string(format!("📋clipboard cb_clipboard_trash"))?;
  let cb_clipboard_trash = Box::new(move || -> Result<()> {
    // let _ = psl.call::<()>(s_lua.clone());
    // warn!("📋clipboard cb_clipboard_trash {}",&cc_paths.len());
    let _ = trash_all(&cc_paths);
    let _ = psl.call::<()>(format!("trash_all s_lua {}",&cc_paths.len()))?;
    let _ = alert.call::<()>("sadfsdf")?;
    // todo: print # of failed by reason and show a list in cacao gui? meanwhile just use the shorter lua wrappers
    Ok(())
  });

  // 4 Ask user confirmation for overwriting (and pass our callback cb_clipboard_trash to actually delete)
  let win_id_objc:ShareId<Object> = get_win_id_objc(&nswin_lua)?; // convert pointer to Objc type
  WM.with(|wm| {
    wm.save_cb(cb_clipboard_trash);
    wm.save_marta(win_id_objc.clone()); //todo: move saving marta to open modal (check if saved)
    wm.open_sheet(win_id_objc);
    wm.on_message(Message::OpenOverwriteSheet); //todo: replace ↑
  });

  /*
  warn!("move_cb_to"); // this creates a new event, outside of any spans.
  // warn!("fn console(in_str)@Marta's Rust lua module es_rs.rs, in_str=‘{:?}’", path_to); // this creates a new event, outside of any spans.

  // let sss:mlua::Value = path_to.into_os_string().into_lua(lua)?;
  // let s_lua:LuaString = lua.create_string(sss)?;
  // let s_lua:LuaString = lua.create_string(path_to.into_lua(lua)?)?;
  // let s_lua:mlua::Value::String = path_to.into_lua(lua)?;
  // let _ = alert.call::<()>(s_lua.clone())?;
  use std::{ffi::OsString, path::PathBuf};
  use uu_mv::{BackupMode, UpdateMode};
  // 🗇🗍🗊🗐 suffix doesn't work with numbered backup
  let overwrite_no                	= false;
  let overwrite_ask               	= false;
  let overwrite = if overwrite_no 	{uu_mv::OverwriteMode::NoClobber
  } else          if overwrite_ask	{uu_mv::OverwriteMode::Interactive
  } else                          	{uu_mv::OverwriteMode::Force};
  let progress_bar                	= true;
  let verbose                     	= false;
  let arg_update                  	= false;
  let update = if arg_update      	{UpdateMode::ReplaceIfOlder
  } else                          	{UpdateMode::ReplaceAll};
  let arg_backup = "no";
  let backup = match arg_backup {
    "no"	=> BackupMode::NoBackup,
    "s" 	=> BackupMode::SimpleBackup,
    "#" 	=> BackupMode::NumberedBackup,
    "x" 	=> BackupMode::ExistingBackup,
    _   	=> BackupMode::NoBackup,
  };
  let suffix = String::from("~");
  let target_dir = Some(path_to.into_os_string());
  //todo: how to do interactive intput from within a gui? redirect stdio?? or something

  let options = uu_mv::Options {overwrite,progress_bar,verbose,suffix,backup,update, target_dir,no_target_dir:false,
    strip_slashes:false, debug:false,};
  let cc_paths_oss = cc_paths.into_iter().map(|p| p.into_os_string()).collect::<Vec<OsString>>();

  if let Err(error) = uu_mv::mv(&cc_paths_oss, &options) {
    let s_lua:LuaString = lua.create_string(format!("❗mv error"))?; let _ = pss.call::<()>(s_lua.clone()); return Ok(s_lua)
    // return Err(ShellError::GenericError {
    //   error: format!("{}", error),
    //   msg: format!("{}", error),
    //   span: None,
    //   help: None,
    //   inner: Vec::new(),
    // });
  }
  */

  Ok(lua.create_string("Ok")?)
}

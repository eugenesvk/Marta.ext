use crate::*;
use crate::libmod::*;

mod win_mgr;
mod message;
mod overwrite;
pub use win_mgr  	::*;
pub use message  	::*;
pub use overwrite	::*;

use cacao::appkit::{App, AppDelegate};
use cacao::appkit::window::{Window,WindowConfig,WindowDelegate, Window as Win, WindowConfig as WinCfg, WindowDelegate as WinDelegate};

use cacao::foundation::{id, nil, to_bool, NSInteger, NSString, NSUInteger, NO, YES, AutoReleasePool};
use objc::runtime::Object;
use objc::{class, msg_send, sel};
use objc_id::{ShareId,Id,Shared};

thread_local! {pub static WM:LazyCell<WinMgr> = LazyCell::new(|| {WinMgr::default()});}

pub fn ask_name(lua:&Lua, (ctx_a, cb_alert_v):(LuaAnyUserData, LuaValue)) -> LuaResult<LuaString> { // move clipboard files to the destination
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

  let nswin_lua 	:LuaLightUserData	= ctx_w.get(ctxW::nsWindow	)?;//LightUserData<NSWindow>, equivalent to an unmanaged raw pointer
  let nswin_rptr	:*mut c_void     	= nswin_lua.0; // get the pointer
  if nswin_rptr.is_null() {let s_lua:LuaString = lua.create_string("✗ nswin_ptr.is_null")?;let _ = alert.call::<()>(s_lua.clone())?;
    return Ok(lua.create_string("📋 got no pointer to the main window, can't create any dialogs…")?)
  }

  let win_id_objc:ShareId<Object> = get_win_id_objc(&nswin_lua)?; // convert pointer to Objc type
  WM.with(|wm| {
    wm.save_marta(win_id_objc.clone()); //todo: move saving marta to open modal (check if saved)
    wm.open_sheet(win_id_objc);
    wm.on_message(Message::OpenOverwriteSheet); //todo: replace ↑
  });

  Ok(lua.create_string("Ok")?)
}

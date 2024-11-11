/// stub function defining some useful Marta API variables, copy&paste and add custom logic
fn fn_stub(lua:&Lua, (ctx_a, path_to):(LuaAnyUserData, PathBuf)) -> LuaResult<LuaString> {
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

  let s_lua:LuaString = lua.create_string("stub for a cut")?;
  Ok(s_lua)
}

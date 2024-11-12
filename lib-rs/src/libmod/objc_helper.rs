pub fn get_win_id_objc(nswin_lua:&LuaLightUserData) -> Result<ShareId<Object>> {
  // nswin = Unmanaged<NSWindow>.fromOpaque(nsWindowPtr).takeUnretainedValue(); //Swift example
  let ptr_r:*mut c_void	= nswin_lua.0;
  if ptr_r.is_null() {return Err(anyhow!("❗📋 got no pointer to the main window, can't create any dialogs…"))}
  let ptr_objc:id = ptr_r.cast(); //type id = *mut Object;
  let win_id_objc:ShareId<Object> = unsafe{ ShareId::from_ptr(ptr_objc) };
  Ok(win_id_objc)
}
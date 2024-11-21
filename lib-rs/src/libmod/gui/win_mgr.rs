//! Windows manager to store reference to Marta's window and all the popup windows we create
use crate	::{*,
  libmod 	::{*,
    gui  	::overwrite::wOverwrite}};

use std::sync::RwLock;
use std::cell::Cell;
use tracing::{info,warn,Level};

use cacao::appkit::window::{Window,WindowConfig,WindowDelegate, Window as Win, WindowConfig as WinCfg, WindowDelegate as WinDelegate};
use cacao::objc::runtime::Object;
use objc_id::{ShareId,Id,Shared};

/// A helper method: check for our modal window existence, and create it if not, then show it
fn open_or_show<W,F>(win_lock:&RwLock<Option<Win<W>>>, vendor:F) where W:WinDelegate+'static, F:Fn()->(WinCfg,W) {
  let mut lock = win_lock.write().unwrap();
  if let    Some(win) = &*lock {              win.show();
  } else {         let (wincfg, windelegate) = vendor();
    let win = Win::with(wincfg, windelegate); win.show();
    *lock = Some(win);}
}

/// A helper method: save Marta's main window if it hasn't been saved it
fn save_win         (win_lock:&RwLock<Option<Win   >>, win_id_objc:ShareId<Object>) {
  let mut lock = win_lock.write().unwrap();
  if let    Some(win) = &*lock {warn!("save_win exists, skip");
  } else {warn!("save_win saving win_marta");
    let win_marta:Win = Win {objc:win_id_objc, delegate:None};
    *lock = Some(win_marta);
  }
}

#[derive(Default)]
pub struct WinMgr {pub marta:RwLock<Option<Win            >>
  ,                pub modal:RwLock<Option<Win<wOverwrite>>>,}
impl       WinMgr {
  pub fn open_new       (&self                             ) {open_or_show(&self.modal, || (WinCfg::default(), wOverwrite::new()));}
  pub fn save_marta     (&self, win_id_objc:ShareId<Object>) {save_win    (&self.marta, win_id_objc);}
  pub fn open_sheet     (&self, win_id_objc:ShareId<Object>) {
    let on_modal_close = || {warn!("WinMgr open_sheet");}; //run once the sheet is dismissed
    let mut lock_modal = self.modal.write().unwrap();
    if let Some(win_modal) = &*lock_modal {warn!("WinMgr open_sheet existing");
      self.begin_sheet(&win_modal, on_modal_close);
    } else {warn!("WinMgr open_sheet NEW modal");
      let mut modal = wOverwrite::new();
      modal.save_marta_ptr(win_id_objc);
      let win_modal = Win::with(WinCfg::default(), modal);
      self.begin_sheet(&win_modal, on_modal_close);
      *lock_modal = Some(win_modal);
    }
  }
  pub fn close_sheet    (&self) {
    let mut lock_modal = self.modal.write().unwrap();
    if let Some(modal) = &*lock_modal {warn!("WinMgr close_sheet exists");
      let   lock_marta = self.marta.write().unwrap();
      if let Some(win_marta) = &*lock_marta {win_marta.end_sheet(&modal);}else{warn!("✗ WinMgr close_sheet does NOT exist, doing nothing…");}
    }else{warn!("✗ WinMgr close_sheet does NOT exist, doing nothing…");}
    *       lock_modal = None;}

  pub fn toggle_full_screen(&self) { // TODO: delete
    let win_lock = &self.marta;
    let lock = win_lock.write().unwrap();
    if let Some(win) = &*lock {win.toggle_full_screen();}
  }

  /// Run modal on marta's main window
  pub fn begin_sheet<W,F>(&self, win:&Window<W>, on_modal_close:F) where W:WinDelegate+'static, F:Fn()+Send+Sync+'static {
    let   lock_marta = self.marta.write().unwrap();
    if   let Some(win_marta) = &*lock_marta {win_marta.begin_sheet(win, on_modal_close);}}

  pub fn on_message(&self, msg:Message) { // moved from Dispatcher since we can't use dispatch for an app we don't own
    match msg {
      Message::OpenOverwriteSheet	=> {warn!("M:OpenOverwriteSheet"	); /*self.open_sheet() 	;*/},
      Message::CloseSheet        	=> {warn!("M:CloseSheet"        	); /*self.close_sheet()	;*/},
      _ => {warn!("M:got other UI msg @ implg WinMgr");}
    }
    if let Some(w) = &*(self.modal.read().unwrap())	{if let Some(delegate) = &w.delegate {warn!("on_msg modal");delegate.on_message(msg.clone());}}
  }
}

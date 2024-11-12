//! The main Todos window
use std::{
  ffi::c_void,
  sync::RwLock};
use tracing::{info,warn,Level,error};

use crate::*;
use crate::libmod::*;

use cacao::appkit::window::{Window       , WindowConfig          , WindowDelegate
  ,                         Window as Win, WindowConfig as WinCfg, WindowDelegate as WinDelegate};
use objc::runtime::Object;
use objc_id::{ShareId,Id,Shared};

// 1 Window
pub struct wOverwrite {pub                               content:ViewController     <vOverwrite>           ,win:Option<Window>, win_id_objc:Option<ShareId<Object>>,}
impl       wOverwrite {pub fn new() -> Self {wOverwrite {content:ViewController::new(vOverwrite::default()),win:None          , win_id_objc:None}}
  pub fn on_message(&self, msg:Message) {
    let win = self.win.as_ref().unwrap();
    match msg { //TODO: test storing window as is
      Message::TestChangeTitle	=> {win.set_title("TestChangeTitle"	);/*win.set_content_view_controller(&self.win1);*/},
      _                       	=> {}
    }
  }

  pub fn save_marta_ptr(&mut self, win_id_objc:ShareId<Object>) {warn!{"impl wOverwrite save_marta_ptr, store a ref to our main windows's pointer for future use"};
    if self.win_id_objc.is_none() {
       self.win_id_objc =
      Some( win_id_objc);
      warn!("SAVED self.win_id_objc {:?}",self.win_id_objc); //Some(
    }
  }
}

// 2 Root view controller
use cacao::{
  layout	::{Layout,LayoutConstraint},
  text  	::{Label,TextAlign},
  view  	::{View,ViewDelegate,ViewController},
};

#[derive(Debug)] pub struct vOverwrite {label:Label}
impl Default      for       vOverwrite {fn default() -> Self {vOverwrite {label:Label::default()}}}
impl ViewDelegate for       vOverwrite {const NAME: &'static str = "vOverwrite";
  fn did_load(&mut self, v:View) { // when the View is ready to work with. You're passed a View - this is safe to store and use repeatedly, but it's not thread safe - any UI calls must be made from the main thread!
    // warn!("ViewDelegate for vOverwrite did_load function");
    self.label.set_text("Label vOverwrite");
    self.label.set_text_alignment(TextAlign::Center);
    v.add_subview(&self.label);

    LayoutConstraint::activate(&[
      self.label.top     	.constraint_equal_to(&v.top     	).offset( 100.),
      self.label.leading 	.constraint_equal_to(&v.leading 	).offset(  16.),
      self.label.trailing	.constraint_equal_to(&v.trailing	).offset( -16.),
      self.label.bottom  	.constraint_equal_to(&v.bottom  	).offset(-100.),
    ]);
  }
}

impl WinDelegate for wOverwrite {const NAME: &'static str = "wOverwrite";
  fn did_load(&mut self, win:Win) { // when this window has loaded in memory, and is about to display (set up your views here)
    // warn!("WinDelegate for wOverwrite did_load function");
    win.set_title                	("wOverwriteTitle"	);
    win.set_autosave_name        	("wOverwrite"     	);
    win.set_minimum_content_size 	(400, 400       	);
    win.set_movable_by_background	(true           	);
    win.set_title                	("Tasks"        	);

    win.set_content_view_controller(&self.content);

    self.win = Some(win);
  }

  fn should_close(&self) -> bool { // when the user has attempted to close the window (not quit the app). Return false here if you need to handle the edge case.
    warn!("WinDelegate for wOverwrite should_close function");
    // self.close_sheet();
    true
  }
  fn cancel(&self) { // close when the ESC key is hit (for modals)
    if let Some(win_id_objc) = &self.win_id_objc {warn!("✓ WinDelegate for wOverwrite cancel function win_id_objc pointer exists {:?} {:?}", self.win_id_objc, win_id_objc);
      WM.with(|wm| {wm.close_sheet();}); //TODO replace with messages?
    } else {warn!("✗ WinDelegate for wOverwrite cancel function win_id_objc pointer does NOT exists {:?}", self.win_id_objc);}
  }
}

use std::{
  ffi::c_void,
  sync::RwLock};
use tracing::{info,warn,Level,error};

use crate::*;
use crate::libmod::*;

use cacao::appkit::window::{Window       , WindowConfig          , WindowDelegate
  ,                         Window as Win, WindowConfig as WinCfg, WindowDelegate as WinDelegate};
use cacao::objc::runtime::Object;
// use objc::runtime::Object;
use objc_id::{ShareId,Id,Shared};

// 1 Window
pub struct wOverwrite {pub                               content:ViewController     <vOverwrite>           ,win:Option<Window>, win_id_objc:Option<ShareId<Object>>,}
impl       wOverwrite {pub fn new() -> Self {wOverwrite {content:ViewController::new(vOverwrite::default()),win:None          , win_id_objc:None}}
  pub fn on_message(&self, msg:Message) {
    let win = self.win.as_ref().unwrap();
    match msg { //TODO: test storing window as is
      Message::TestChangeTitle	=> {win.set_title("TestChangeTitle"	);/*win.set_content_view_controller(&self.win1);*/},
      _                       	=> {warn!{"M@wOverwrite:other"};}
    }
  }

  pub fn save_marta_ptr(&mut self, win_id_objc:ShareId<Object>) {warn!{"impl wOver_write save_marta_ptr, store a ref to our main windows's pointer for future use"};
    if self.win_id_objc.is_none() {
       self.win_id_objc =
      Some( win_id_objc);
      warn!("SAVED self.win_id_objc {:?}",self.win_id_objc); //Some(
    }
  }
}

// 2 Root view controller
use cacao::{
  layout 	::{Layout,LayoutConstraint,},
  text   	::{Label,TextAlign,Font,AttributedString, AttributedString as RichStr },
  view   	::{View,ViewDelegate,ViewController,},
  switch 	::Switch,
  button 	::{Button,BezelStyle, BezelStyle as Border,ImagePosition,},
  control	::{Control,ControlSize,},
  color  	::{Color, Theme,},
  image  	::{Image,MacSystemIcon,SFSymbol},
  utils  	::os::OS_VERSION,
};
use cacao::appkit::FocusRingType;
use cacao::objc::{class, msg_send, sel, sel_impl};
use core_graphics::base::CGFloat;

#[derive(Debug,Default)] pub struct vOverwrite {
  pub v       	: View          	,//
  pub title   	: Label         	,//
  pub subtitle	: Label         	,//
  pub subtitle	: Label         	,//
  pub bYes    	: Option<Button>	,//option to allow default derive
  pub bNo     	: Option<Button>	,//
}
// impl Default      for       vOverwrite { /// Creates and returns a stock Overwrite view
//   fn default() -> Self {
//     let v       	= View::new();
//     let title   	= Label::new()	;//v.add_subview(&title   	);
//     let subtitle	= Label::new()	;//v.add_subview(&subtitle	);

//     let mut y=Button::new("❗Overwrite"	);y.set_action(|_| dispatch_ui(Message::MoveOverwrite	));y.set_key_equivalent("\r");//🖍
//     let mut n=Button::new("Cancel"    	);n.set_action(|_| dispatch_ui(Message::MoveCancel   	));n.set_key_equivalent("a");
//     y.set_highlighted(false);y.set_bezel_style(BezelStyle::RegularSquare);y.set_control_size(ControlSize::Large);y.set_text_color(Color::SystemRed  );
//     n.set_highlighted(true );n.set_bezel_style(BezelStyle::RegularSquare);n.set_control_size(ControlSize::Large);n.set_text_color(Color::SystemBlack);
//     v.add_subview(&y);let bYes = y;
//     v.add_subview(&n);let bNo  = n;

//     LayoutConstraint::activate(&[
//       // title   	.top     	.constraint_equal_to(&v.top          	).offset(  2.),
//       // subtitle	.top     	.constraint_equal_to(&title.bottom   	),
//       // // bYes 	.top     	.constraint_equal_to(&subtitle.bottom	).offset(-20.),
//       // // bNo  	.top     	.constraint_equal_to(&subtitle.bottom	).offset(-20.),
//       // title   	.leading 	.constraint_equal_to(&v.leading      	),
//       // subtitle	.leading 	.constraint_equal_to(&v.leading      	),
//       // // bYes 	.leading 	.constraint_equal_to(&v.leading      	).offset( -4.),
//       // // bNo  	.leading 	.constraint_equal_to(&v.leading      	).offset( -4.),
//       // title   	.trailing	.constraint_equal_to(&v.trailing     	),
//       // subtitle	.trailing	.constraint_equal_to(&v.trailing     	),
//       // title   	.width   	.constraint_equal_to_constant(248.   	),
//       // subtitle	.width   	.constraint_equal_to_constant(248.   	),
//       // // bYes 	.width   	.constraint_equal_to_constant(248.   	),bYes	.height	.constraint_equal_to_constant(148.	),
//       // // bNo  	.width   	.constraint_equal_to_constant(248.   	),
//       // subtitle	.bottom  	.constraint_equal_to(&v.bottom       	),
//       // // bYes 	.bottom  	.constraint_equal_to(&v.bottom       	),
//       // bNo     	.bottom  	.constraint_equal_to(&v.bottom       	),

//       v.width.constraint_equal_to_constant(600.0),
//       v.height.constraint_equal_to_constant(500.0),
//       bYes	.width   	.constraint_equal_to_constant(28.	),
//       bYes	.height  	.constraint_equal_to_constant(28.	),
//       bYes	.top     	.constraint_equal_to(&v.top).offset(-1.),
//       bYes	.leading 	.constraint_equal_to(&v.leading).offset(40.),
//       bYes	.trailing	.constraint_equal_to(&v.trailing).offset(-2.),
//       bYes	.bottom  	.constraint_equal_to(&v.bottom).offset(-1.),
//     ]);
//     vOverwrite {v,title,subtitle,bYes,bNo}
//   }
// }
// impl                        vOverwrite {
//   /// Configures vOverwrite, handler is fired on each state change of the checkbox
//   pub fn configure<F>(&mut self, text:&str, subtitle:&str, state:bool, handler:F) where F:Fn()+Send+Sync+'static {
//     self.title   	.set_text(text);
//     self.subtitle	.set_text(subtitle);
//     self.switch  	.set_action(handler);
//     self.switch  	.set_checked(state);
//   }
// }

pub fn toggle_do_nothing() {}
impl ViewDelegate for       vOverwrite {const NAME: &'static str = "vOverwrite";
  fn did_load(&mut self, v:View) { //View is ready to work with, arg View is safe to store and use repeatedly, but it's not thread safe - any UI calls must be made from the main thread!
    warn!("ViewDelegate for vOverwrite did_load function");
    self.title      	.set_text          	("title_Label_vOverwrite"   	);
    self.subtitle   	.set_text          	("subtitle_Label_vOverwrite"	);
    // self.subtitle	.set_text_alignment	(TextAlign::Center          	);
    v.add_subview(&self.v);



    let title   	= Label::new()	;//v.add_subview(&title   	);
    let subtitle	= Label::new()	;//v.add_subview(&subtitle	);

    let mut y=Button::new("❗Overwrite"	);y.set_action(|_| dispatch_ui(Message::MoveOverwrite	));y.set_key_equivalent("\r");//🖍
    let mut n=Button::new("Cancel"    	);n.set_action(|_| dispatch_ui(Message::MoveCancel   	));n.set_key_equivalent("a");
    y.set_highlighted(false);y.set_bezel_style(BezelStyle::RegularSquare);y.set_control_size(ControlSize::Large);y.set_text_color(Color::SystemRed  );
    n.set_highlighted(true );n.set_bezel_style(BezelStyle::RegularSquare);n.set_control_size(ControlSize::Large);n.set_text_color(Color::SystemBlack);
    v.add_subview(&y);let bYes = y;
    v.add_subview(&n);let bNo  = n;

    LayoutConstraint::activate(&[
      // title   	.top     	.constraint_equal_to(&v.top          	).offset(  2.),
      // subtitle	.top     	.constraint_equal_to(&title.bottom   	),
      // // bYes 	.top     	.constraint_equal_to(&subtitle.bottom	).offset(-20.),
      // // bNo  	.top     	.constraint_equal_to(&subtitle.bottom	).offset(-20.),
      // title   	.leading 	.constraint_equal_to(&v.leading      	),
      // subtitle	.leading 	.constraint_equal_to(&v.leading      	),
      // // bYes 	.leading 	.constraint_equal_to(&v.leading      	).offset( -4.),
      // // bNo  	.leading 	.constraint_equal_to(&v.leading      	).offset( -4.),
      // title   	.trailing	.constraint_equal_to(&v.trailing     	),
      // subtitle	.trailing	.constraint_equal_to(&v.trailing     	),
      // title   	.width   	.constraint_equal_to_constant(248.   	),
      // subtitle	.width   	.constraint_equal_to_constant(248.   	),
      // // bYes 	.width   	.constraint_equal_to_constant(248.   	),bYes	.height	.constraint_equal_to_constant(148.	),
      // // bNo  	.width   	.constraint_equal_to_constant(248.   	),
      // subtitle	.bottom  	.constraint_equal_to(&v.bottom       	),
      // // bYes 	.bottom  	.constraint_equal_to(&v.bottom       	),
      // bNo     	.bottom  	.constraint_equal_to(&v.bottom       	),

      v.width.constraint_equal_to_constant(600.0),
      v.height.constraint_equal_to_constant(500.0),
      bYes	.width   	.constraint_equal_to_constant(28.	),
      bYes	.height  	.constraint_equal_to_constant(28.	),
      bYes	.top     	.constraint_equal_to(&v.top).offset(-1.),
      bYes	.leading 	.constraint_equal_to(&v.leading).offset(40.),
      bYes	.trailing	.constraint_equal_to(&v.trailing).offset(-2.),
      bYes	.bottom  	.constraint_equal_to(&v.bottom).offset(-1.),
    ]);
    self.bYes = Some(bYes);
    self.bNo  = Some(bNo);
    // vOverwrite {v,title,subtitle,bYes,bNo}







    /*
    // LayoutConstraint::activate(&[
      // v.width.constraint_equal_to_constant(1000.0),

      // self.bYes	.top     	.constraint_equal_to(&v.top).offset(-2.),
      // self.bYes	.leading 	.constraint_equal_to(&v.leading).offset(-2.),
      // self.bYes	.trailing	.constraint_equal_to(&v.trailing).offset(-2.),
      // self.bYes	.width   	.constraint_equal_to_constant(28.	),
      // self.bYes	.height  	.constraint_equal_to_constant(28.	),
      // self.bYes	.bottom  	.constraint_equal_to(&v.bottom).offset(-1.)

      // self.bYes	.top     	.constraint_equal_to(&self.subtitle.bottom).offset(1.),
      // self.bYes	.trailing	.constraint_equal_to(&v.trailing).offset(-40.),
      // self.bYes	.width   	.constraint_equal_to_constant(248.	),
      // self.bYes	.bottom  	.constraint_equal_to(&v.bottom).offset(-10.)
    // ]);
    // LayoutConstraint::activate(&[
    //   self.label.top     	.constraint_equal_to(&v.top     	).offset( 100.),
    //   self.label.leading 	.constraint_equal_to(&v.leading 	).offset(  16.),
    //   self.label.trailing	.constraint_equal_to(&v.trailing	).offset( -16.),
    //   self.label.bottom  	.constraint_equal_to(&v.bottom  	).offset(-100.),
    // ]);
    */
  }
}

impl WinDelegate for wOverwrite {const NAME: &'static str = "wOver_write";
  fn did_load(&mut self, win:Win) { // when this window has loaded in memory, and is about to display (set up your views here)
    // warn!("WinDelegate for wOverwrite did_load function");
    win.set_title                  	("wOver_writeTitle"	);
    win.set_autosave_name          	("wOver_write"     	);
    // win.set_minimum_content_size	(100, 100         	);//min size this window can shrink to, no max
    win.set_minimum_size           	(600, 500         	);//min/max match content
    win.set_movable_by_background  	(true             	);

    win.set_content_view_controller(&self.content);

    self.win = Some(win);
  }

  fn should_close(&self) -> bool { // when the user has attempted to close the window (not quit the app). Return false here if you need to handle the edge case.
    warn!("WinDelegate for wOver_write should_close function");
    // self.close_sheet();
    true
  }
  fn cancel(&self) { // close when the ESC key is hit (for modals)
    if let Some(win_id_objc) = &self.win_id_objc {warn!("✓ WinDelegate for wOver_write cancel function win_id_objc pointer exists {:?} {:?}", self.win_id_objc, win_id_objc);
      WM.with(|wm| {wm.close_sheet();}); //TODO replace with messages?
    } else {warn!("✗ WinDelegate for wOver_write cancel function win_id_objc pointer does NOT exists {:?}", self.win_id_objc);}
  }
}

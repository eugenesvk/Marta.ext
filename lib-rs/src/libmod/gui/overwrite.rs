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
pub struct wOverwrite {pub                               content:ViewController     <vOverwrite>           ,win:Option<Window>, win_id_objc:Option<ShareId<Object>>,key_monitor:RwLock<Option<EventMonitor>>,}
impl       wOverwrite {pub fn new() -> Self {wOverwrite {content:ViewController::new(vOverwrite::default()),win:None          , win_id_objc:None                   ,key_monitor:RwLock::new(None)}}
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

  pub fn start_monitoring(&self) {
    let mut lock = self.key_monitor.write().unwrap();
    *lock = Some(Event::local_monitor(EventMask::KeyDown | EventMask::KeyUp | EventMask::FlagsChanged, |evt| {
      //use calculator::{dispatch, Msg};
      let kind = evt.kind();
      let ev_t:&str = match kind {
        EventType::FlagsChanged	=> &format!("Δ in {}{}{}{}{}{:#}",&Mod::CapsLock,&Mod::Shift,&Mod::Control,&Mod::Option,&Mod::Command,&Mod::Function),
        EventType::KeyDown     	=> "↓",
        EventType::KeyUp       	=> "↑",
        _                      	=> "?",
      };
      match evt.kind() {
         EventType::KeyDown
        |EventType::KeyUp	=> {
          let chars = evt.characters(); //characters associated with a key-up or key-down event
          let chars = evt.characters_ignoring_modifiers(); //characters associated with a key-up or key-down event w/o mods (except ⇧)
          let key_code = evt.key_code(); //virtual code for the key associated with the event.
          let mod_flag = evt.modifier_flags(); //modifier flags for the key associated with the event.
          let bits = str::replace(&format!("{: >24b}",mod_flag.bits()),"0"," ");
          warn!("{} {}𝚻{:?} vk={} mod_flag={}\tbits=0b{}", chars, ev_t,kind, key_code, mod_flag,bits);
          match chars.as_ref() {
            "y" => {press_y("letter y")},
            "c" => {press_n("letter c")},
            "s" => {press_n("letter s")},
            _ => return Some(evt),
          }
        },
        // use key code to diff ‹vs› in modifiers as key presses (not as part of modifier flags)
        EventType::FlagsChanged	=> {
          let key_code = evt.key_code(); //virtual code for the key associated with the event.
          let mod_flag = evt.modifier_flags(); //modifier flags for the key associated with the event.
          let bits = str::replace(&format!("{: >24b}",mod_flag.bits()),"0"," ");
          warn!("   {}𝚻{:?} vk={} mod_flag={:#}\tbits=0b{}", ev_t,kind, key_code, mod_flag,bits);
        }
        _	=> {//dbg!("  𝚻{:?} ev_t={} ev={:?}", kind, ev_t, evt);
          return None},
      }
      None
    }));
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
  objc   	::{class, msg_send, sel, sel_impl},
  appkit 	::{Event, EventMask, EventMonitor, FocusRingType, EventModifierBitFlag, EventModifierBitFlag as Mod},
};
use core_graphics::base::CGFloat;
use core::ops::Range;

pub fn toggle_do_nothing() {}
fn press_y(s:&str) {println!("Y action from: {}",s)}
fn press_n(s:&str) {println!("N action from: {}",s)}

#[derive(Debug,Default)] pub struct vOverwrite {
  pub v       	: View          	,//
  pub title   	: Label         	,//
  pub subtitle	: Label         	,//
  pub bYes    	: Option<Button>	,//option to allow default derive
  pub bNo     	: Option<Button>	,//
}

use cacao::events::EventType;
impl ViewDelegate for       vOverwrite {const NAME: &'static str = "vOverwrite delegate";
  fn did_load(&mut self, v:View) { //View is ready to work with, arg View is safe to store and use repeatedly, but it's not thread safe - any UI calls must be made from the main thread!
    warn!("did_load@ViewDelegate for vOver_write");
    let dynamic = Color::dynamic(|style| match (style.theme, style.contrast) {
      (Theme::Dark, _)	=> Color::SystemGreen,
      _               	=> Color::SystemRed});


    self.title      	.set_text          	("title_Label_vOver_write"   	);
    self.subtitle   	.set_text          	("subtitle_Label_vOver_write"	);
    // self.subtitle	.set_text_alignment	(TextAlign::Center           	);
    // v.add_subview(&self.v);

    let title   	= Label::new()	;v.add_subview(&self.title   	);
    let subtitle	= Label::new()	;v.add_subview(&self.subtitle	);



    // let mut y=Button::new("O̲verwrite"	);y.set_action(|_| {press_y("UI button")});y.set_key_equivalent("o"); //❗
    // let mut n=Button::new("S̲kip"     	);n.set_action(|_| {press_n("UI button")});n.set_key_equivalent("\r");

    // Add colored button labels, highlighting the first accelerator underlined letter via rich string formatting
    let lbl = "S̲kip"; let lbl_u16 = lbl.encode_utf16(); let lbl_len = lbl_u16.count() as isize;
    let acc = "S̲"   ; let acc_u16 = acc.encode_utf16(); let acc_len = acc_u16.count() as isize;
    let mut n=Button::new(lbl	);n.set_action(|_| {press_n("UI button")});n.set_key_equivalent("\r");
    let mut attr_str = RichStr::new(lbl);
    let font = Font::system(16.); attr_str.set_font(font, Range{start:0,end:lbl_len}); // make label bigger
    let accelerator = Range{start:0,end:acc_len}; //[start,end)
    attr_str.set_text_color(cacao::color::Color::rgb(150,255,150), accelerator.clone());
    let font = Font::bold_system(16.);attr_str.set_font(font, accelerator);
    n.set_attributed_text(attr_str);

    let lbl = "O̲verwrite"; let lbl_u16 = lbl.encode_utf16(); let lbl_len = lbl_u16.count() as isize;
    let acc = "O̲"        ; let acc_u16 = acc.encode_utf16(); let acc_len = acc_u16.count() as isize;
    let mut y=Button::new(lbl	);y.set_action(|_| {press_y("UI button")});y.set_key_equivalent("o");
    let mut attr_str = RichStr::new(lbl);
    let font = Font::system(16.); attr_str.set_font(font, Range{start:0,end:lbl_len}); // make label bigger

    let accelerator = Range{start:0,end:acc_len}; //[start,end)
    attr_str.set_text_color(cacao::color::Color::rgb(200,0,0), accelerator.clone());
    let font = Font::bold_system(16.);attr_str.set_font(font, accelerator);
    y.set_attributed_text(attr_str);

    y.set_control_size(ControlSize::Large);
    n.set_control_size(ControlSize::Large);
    y.set_bezel_style(BezelStyle::Rounded);
    n.set_bezel_style(BezelStyle::Rounded); // RegularSquare, ShadowlessSquare,SmallSquare,TexturedSquare break become vertical 100% of the height
    y.set_focus_ring_type(FocusRingType::Exterior); // seems to have no effect
    n.set_focus_ring_type(FocusRingType::None); // already an highlighted button, don't need another indicator
    // y.set_text_color(Color::SystemRed  );
    // n.set_text_color(Color::SystemBlack);

    if let os_info::Version::Semantic(os_major,_,_) = OS_VERSION.version() {
      if *os_major >= 11 {//debug!("info major version={:?}", os_major);
        let icon = Image::symbol(SFSymbol::SquareAndArrowDownOnSquareFill, "Overwrite"); //SFSymbol min version 11, alt MacSystemIcon
        y.set_image(icon);
        y.set_image_position(ImagePosition::ImageLeft); // developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Button/Tasks/SettingButtonImage.html
      }
    }
    v.add_subview(&y);
    v.add_subview(&n);

    ////win.set_content_view(&self.content);

    // Add colored button shortcut reminder labels using rich text formatting
    // let yl	= Label::new();yl.set_text("y")  	;v.add_subview(&yl	);
    // let nl	= Label::new();nl.set_text("↩¦c")	;v.add_subview(&nl	);
    let lbl = "y"; let lbl_u16 = lbl.encode_utf16(); let len = lbl_u16.count() as isize;
    let acc = "y"; let acc_len = acc.encode_utf16().count() as isize;
    let yl=Label::new();
    let mut attr_str = RichStr::new(lbl);
    // let font = Font::system(16.); attr_str.set_font(font, Range{start:0,end:len}); // make label bigger
    let accelerator = Range{start:0,end:acc_len}; //[start,end)
    attr_str.set_text_color(cacao::color::Color::rgb(240,140,40), accelerator.clone());
    // let font = Font::bold_system(16.); attr_str.set_font(font, accelerator);
    yl.set_attributed_text(attr_str);
    v.add_subview(&yl	);

    let lbl = "↩"; let lbl_u16 = lbl.encode_utf16(); let len = lbl_u16.count() as isize;
    let acc = "↩"; let acc_len = acc.encode_utf16().count() as isize;
    let nl1=Label::new();
    let mut attr_str = RichStr::new(lbl);
    // let font = Font::system(16.); attr_str.set_font(font, Range{start:0,end:len}); // make label bigger
    let accelerator = Range{start:0,end:acc_len}; //[start,end)
    attr_str.set_text_color(cacao::color::Color::rgb(240,140,40), accelerator.clone());
    // let font = Font::bold_system(16.); attr_str.set_font(font, accelerator);
    nl1.set_attributed_text(attr_str);
    v.add_subview(&nl1	);

    let lbl = "c"; let lbl_u16 = lbl.encode_utf16(); let len = lbl_u16.count() as isize;
    let acc = "c"; let acc_len = acc.encode_utf16().count() as isize;
    let nl2=Label::new();
    let mut attr_str = RichStr::new(lbl);
    // let font = Font::system(16.); attr_str.set_font(font, Range{start:0,end:len}); // make label bigger
    let accelerator = Range{start:0,end:acc_len}; //[start,end)
    attr_str.set_text_color(cacao::color::Color::rgb(240,140,40), accelerator.clone());
    // let font = Font::bold_system(16.); attr_str.set_font(font, accelerator);
    nl2.set_attributed_text(attr_str);
    v.add_subview(&nl2	);

    let hn:f64 = 20.0; let hy:f64 = hn; //20 seems to be the default large, but manually setting.height makes the buttons bug and have diff H
    LayoutConstraint::activate(&[
      n  	.top     	.constraint_equal_to(&v.top       	).offset( 46.),
      nl1	.top     	.constraint_equal_to(&n.top       	),
      nl2	.bottom  	.constraint_equal_to(&n.bottom    	),
      y  	.top     	.constraint_equal_to(&v.top       	).offset( 46.),
      yl 	.top     	.constraint_equal_to(&y.top       	).offset(- 3.),
      n  	.bottom  	.constraint_equal_to(&v.bottom    	).offset(-16.),
      y  	.bottom  	.constraint_equal_to(&v.bottom    	).offset(-16.),
      n  	.leading 	.constraint_equal_to(&v.leading   	).offset( 16.),y	.leading	.constraint_greater_than_or_equal_to(&n.trailing	).offset(5.),
      nl1	.right   	.constraint_equal_to(&n.right     	).offset(- 2.),
      nl2	.right   	.constraint_equal_to(&n.right     	).offset(- 2.),
      y  	.trailing	.constraint_equal_to(&v.trailing  	).offset(-46.),
      yl 	.right   	.constraint_equal_to(&y.right     	).offset(- 2.),
      n  	.width   	.constraint_equal_to_constant(200.	)             ,//n	.height	.constraint_equal_to_constant(hn	),
      y  	.width   	.constraint_equal_to(&n.width     	)             ,//y	.height	.constraint_equal_to_constant(hy	),
    ]);
    self.bYes = Some(y);
    self.bNo  = Some(n);
    self.v  = v;
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
    self.start_monitoring(); // Event Monitors needs to be started after the Window is activated
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

//! Messages that we used to thread control throughout the application

use crate	::{*,
  libmod 	::*};
use tracing::{debug,info,warn,Level,error};
use cacao::appkit::App;

/// Messages filter from the parents to children, but cacao's dispatch is broken since we don't own the main app from this extension, this is mostly left as to make the architecture similar
#[derive(Clone,Debug)] pub enum Message {
  /// (Re)Open the main window
  OpenMainWindow,
  /// Open a new confirmation modal sheet window
  OpenOverwriteSheet,
  /// Close the current active sheet
  CloseSheet,
  /// Test
  TestChangeTitle,
  /// Button Yes
  ActOnBtnYes,
  /// Button No
  ActOnBtnNo,
}

/// Dispatch a message to the window manager (can't do regular app background thread dispatch since we don't control the app)
pub fn dispatch_ui(msg:Message) {debug!("'Dispatching' UI message: {:?}", msg);
  WM.with(|wm| {wm.on_message(msg);});
}

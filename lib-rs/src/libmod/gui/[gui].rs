use crate::*;
use crate::libmod::*;

mod win_mgr;
mod message;
mod overwrite;
pub use win_mgr  	::*;
pub use message  	::*;
pub use overwrite	::*;

thread_local! {pub static WM:LazyCell<WinMgr> = LazyCell::new(|| {WinMgr::default()});}

use std::env;
use crate::*;
use crate::libmod::*;
use std::path::PathBuf;

use xattr;
use anyhow::{anyhow, Result, bail};

static xattr_key	:&str = "es_cut_orig_path";
static TRASH    	:&str = ".Trash";

static _dbg:i8 = 1;
/// Quick and dirty way to disable blocks of debug-level code, use `if _d(1) {}` to do something only if global _dbg ≥ 1
pub fn _d(lvl:i8) -> bool {if _dbg>=lvl{true}else{false}}

use std::collections::HashMap;

pub fn trash_all(cc_paths:& Vec<PathBuf>) -> Result<()> {
  // let mut skipped:HashMap<String,Vec<PathBuf>> = HashMap::new(); //todo push lists later, group by error type
  let mut skipped_nm   :Vec<PathBuf> = vec![]; // skipped due to unresolved file name
  let mut skipped_par  :Vec<PathBuf> = vec![]; // skipped due to unresolved parent or canonicalization error
  let mut skipped_trash:Vec<PathBuf> = vec![]; // skipped due to already being in trash

  #[allow(deprecated)]
  let home_dir = match env::home_dir() {
    Some(path)	=> path,
    None      	=> bail!("Impossible to get your home dir!"),
  };
  let mut path_counts:HashMap<PathBuf,u8> = HashMap::new();
  let batch_fd = 1;
  for path in cc_paths {
    let file_name = match path.file_name() {
      Some(p)    	=> p,
      None       	=> {skipped_nm.push(path.into()); continue}};
    let parent   	= match path.parent      () {
      Some(p)    	=> {
        let can  	= match p   .canonicalize() {
          Ok (pc)	=> pc,
          Err(e )	=> {skipped_par.push(path.into()); continue}};
        can },
      None	=> {skipped_par.push(path.into()); continue}};

    let mut p = PathBuf::new();p.push(home_dir   .clone());p.push(TRASH);  let trashed_par   = p;
    let mut p = PathBuf::new();p.push(trashed_par.clone());p.push(file_name); let trashed_path  = p;
    let mut p = PathBuf::new();p.push(parent);             p.push(file_name); let resolved_path = p; // Resolve sym🔗 in dirs, but not the file itself
    if _d(1){warn!("trashed_path = {:?} resolved_path={:?}",trashed_path,resolved_path);}

    if resolved_path.starts_with(trashed_par) {skipped_trash.push(path.into()); continue} // already in trash

    // let trashed_file = None;
    warn!("deleting path from cb_clipboard_trash {:?}",path);
  }
  Ok(())
}

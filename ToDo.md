# rust
  - how to replicate these C example build commands
    - `-arch arm64 -arch x86_64` build a universal binary that runs both on Intel-based Macs and Macs with an Apple Silicon chip
    - `mmacosx-version-min=10.12` builds a binary compatible with all macOS versions supported by Marta (for Marta 0.8.1, it is macOS Sierra);
  - create Marta template plugin with various helper function and init.lua loader

# es¦cut
- add a way to cancel long operation (a button? in a progress bar)
- TODO: verify that did_load doesn't recreate all the views on every single invocation of the dialog
- allow quitting the app even when modal sheet is opened [src](https://forums.macrumors.com/threads/terminate-while-nssheet-isvisible-solved.522228/)
- dynamic: change modal color scheme to read from Marta's theme file or have a separate file that does the same and read its name from marta's config, pass the value to the function and cache the values in a global oncecell config var
- UI window
  - split 3 red/yellow/green buttons, make the colors much more muted and move close to the right corner as it's a categorically different button from min/max
- UI button
  - can't highlight on mouseover? [fr](https://github.com/ryanmcgrath/cacao/issues/140), add mouseover with help and e.g. shortcut on a button should show a different style
  - can't highlight to simulate a button press manually when manual accelerators are used?
  - make default width higher than normal, but not constrained
  - ? disable beeps on key presses that aren't bound as keybinds (need to subclass something)?
  - ? check how to make very constrained view still visible: buttons should shrink to a single icon with 0 padding
  + (use unicode underscore + AttributedString) button highlight acelerator key in a different user-customizable color (no mnemonics, deprecated)
  + button: can I have multiple setKeyEquivalent? seems like no, but maybe if I handle input directly, I could(+)
  + file a bug: RegularSquarel ShadowlessSquare,SmallSquare,TexturedSquare break become vertical 100% of the height
  + add icons to buttons instead of symbols
- UI keyboard
  - ? see [src](https://gist.github.com/rdev/627a254417687a90c493528639465943) SwiftUI modifier to handle customizable keyboard shortcuts anywhere in the app
  - ? differentiate left vs right modifier keys in menu shortcuts
  - ? impossible without more advanced input tracking? NSEvents are dumb and only track modifier status change, but don't report keydown/up events for modifier keys, so when you release ⇪, no modifier state is changed, so no event gets generated. how to track caps lock down/up events? can only track flagchanged, but capslock down can both turn on and off, so not like shift [q](https://github.com/ryanmcgrath/cacao/issues/141)
  - ? get var isARepeat: Bool https://developer.apple.com/documentation/appkit/nsevent
  + keys: differentiate left vs right modifiers, onflgachanged with EventModifierFlag doesn't do that
- replace warn! with debug!
  - OS logging doesn't offer max level, so add it manually patch the logger
- let win_marta:Win = Win {objc:win_id_objc, delegate:None}; // TODO: how to get delegate to an existing window??? Or I don't need it?
- SAFETY: is this the proper way to get lightuserata window id we get from Marta API `fn get_win_id_objc(nswin_lua:&LuaLightUserData) -> Result<ShareId<Object>>`
  - what's the dfiference betwen cast and `let objc:ShareId<Object> = unsafe{ ShareId::from_retained_ptr(nswin_rptr as *mut Object) };`
  // TODO: how to get a ref to the NSWindow and attach our view to it?
    // see https://github.com/ryanmcgrath/cacao/issues/58
    // see https://marta.sh/api/tutorials/presenting-ui-with-swift/
- make a sheet non-modal so you cna browse Marta while deciding whether to overwrite the files
  - but then check on startup whether a warning exists and if it does don't start the next batch of operations?
- write a common function to accept ctxA as userdata to do all the logic in Rust, see serde example https://github.com/mlua-rs/mlua/blob/main/examples/serialize.rs
  - `pub fn move_cb_to(lua:&Lua, (ctx_a,path_to):(LuaAnyUserData, PathBuf)) -> LuaResult<LuaString> {} // move clipboard files to the destination`
  - see https://github.com/mlua-rs/mlua/discussions/123 lifetime hell issue
  - https://github.com/mlua-rs/mlua/discussions/283
  - (the opposite direction) [src](https://github.com/ildar/lua-module-calloop) exposing external crate's (APIs) to Lua world (as a module)
- how to avoid TOCTO?? vuln?
- add hidden actions to press ok/abort buttons in the gui overwrite confirmation menu
- moden overwrite confirmation menu based on Marta theme
- check if current pane folder path exists and is a folder
  - ignore zip files etc, that should be handled by marta in the future
  - show status error and console error if not
- check if clipboar dcontains files, if not: send a message status (no console log)
- how to fast move files in rust?
  - move all without overwriting, if exists, store the other files in a separate vector
  - ask for user input to overwrite files remaining
- show indicator how many files were replaced
  - is there a Marta API for that? unlikely
- ? add an alternative parsing of a list with newlines since 'core.path' doesn't copy file to the clipboard, but a simple string, so clipboard type is NOT "file" and can't be detected as such
  - no, just force the user to user the appropriate clibpoard api
- ? is there a way to attach a is_cut flag to the clipboard object itself to reliably track our cut commands?


# es¦file_link
  - add commands without symbols for opposite tabs
  - also add an option to not add symbols for opposite tabs
  - validation of configs on load to use the proper icons https://github.com/marta-file-manager/marta-issues/issues/983
  - add a 2-view version, creating the link in the 2nd view
  - add a command to choose the type of link and location from the gui not to remember all keybinds
  - missing API to create a hard link, using `ln` in the interim
  - missing API to create an alias, use [alisma](https://eclecticlight.co/taccy-signet-precize-alifix-utiutility-alisma/) in the interim
  - missing API, remove the replacement↓ when it's added
    - tgtStem	= tgtFI.nameWithoutExtension                    -- fails
    - tgtStem	= parentFd:append(tgtName).nameWithoutExtension -- temp replacement
  - unicode examples bug https://github.com/marta-file-manager/marta-issues/issues/975
  - ?add an option to skip hardlinks like sym/alias (no point?)?
  - ?create a symlink by calling Marta action instead (currently can't pass args to marta actions)
    - ctxW:runAction(actG:getById("core.symlink"),ctxPA)
# es¦tab_✗dupe
  - remove my fix (activate our original active tab) when this bug is fixed https://github.com/marta-file-manager/marta-issues/issues/969
  - "Cmd+W"	"es¦tab.✗tab_n_dupe"
    - to ✗close the current tab, select 1 tab to the ←, ✗close all duplicate tabs in the active view
    - current tab has priority over tabs with lower index (to the left)
# init.lua
  - align columns?

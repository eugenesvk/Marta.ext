# rust
  - how to replicate these C example build commands
    - `-arch arm64 -arch x86_64` build a universal binary that runs both on Intel-based Macs and Macs with an Apple Silicon chip
    - `mmacosx-version-min=10.12` builds a binary compatible with all macOS versions supported by Marta (for Marta 0.8.1, it is macOS Sierra);

# es¦cut
- save is_cut global bool var tht would allow the following Paste_cut commands to know whether the clipboard content files comes from a previous Cut command or something else
  - how to edit this global from rust?
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

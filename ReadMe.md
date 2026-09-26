<p align="center">
Plugins, themes, and keybinds for
<br>
<img src="https://marta.sh/images/icon_128.png" alt="Marta logo" width="48" height="48"/><strong>Marta</strong>
</p>
<p align="center">  
File Manager for macOS
</p>

## Plugins

### $\textcolor{blue}{\textrm{1}}\textcolor{grey}{\textrm{.}}$ Link
Create 🔗Symbolic Links, ⤻Aliases, or ⤑Hard Links __in the same pane__ or __at the opposite/inactive pane__ with automatically generated names with:

  - user-configurable affixes (__Symlink🔗.txt__, __Alias⤻.app__, __Hardlink⤑.md__), single-char by default to delete easier when you move the links elsewhere
  - user-configurable affix location (__⎀pre.ext__, __stem⎀.ext__,  __post.ext⎀__)

To customize the plugin behavior, paste the following configuration lines to you `behavior { actions {` section and change them; the built-in [config editor](https://marta.sh/docs/configuration/editor/)[^1] contains helpful tips on what each option does and what values it accepts
<details>
  <summary>Click to expand config</summary>

```
  "es¦file.link.affixSym"  	"🔗"
  "es¦file.link.affixAlias"	"⤻"
  "es¦file.link.affixHard" 	"⤑"
  "es¦file.link.spot"      	"stem"
  "es¦file.link.maxLinkNo" 	1
  "es¦file.link.maxIterNo" 	0
  "es¦file.link.binAlias"  	"/usr/local/bin/alisma"
  "es¦file.link.binHard"   	"/usr/local/opt/coreutils/libexec/gnubin/ln"
```

</details>

['Link' plugin file](https://github.com/eugenesvk/Marta.ext/blob/main/Plugins/es¦file_link.lua)

### $\textcolor{blue}{\textrm{2}}\textcolor{grey}{\textrm{.}}$ Tab deduplication
Close all duplicate tabs (except for the currently active one): 

  - semi-automatically: by remapping <kbd>⌘</kbd><kbd>W</kbd> (`"Cmd+W" "es¦tab.✗tab_n_dupe"`) to
    + ✗close the current tab
    + select 1 tab to the ←left (fixing [this issue](https://github.com/marta-file-manager/marta-issues/issues/967))
    + ✗close all dupe tabs
  - manually: by running the `Tab: ✗Close Duplicates` action[^2] 

['Tab deduplication' plugin file](https://github.com/eugenesvk/Marta.ext/blob/main/Plugins/es¦tab_✗dupe.lua)

### $\textcolor{blue}{\textrm{3}}\textcolor{grey}{\textrm{.}}$ Open alias
Opens ⤻Aliases to folders in Marta instead of the ≝handler like Finder ([bug report](https://github.com/marta-file-manager/marta-issues/issues/845)). Also works with multiple folder selections (including non-aliased) by opening them in new tabs unlike Marta's default open command, which only opens the first folder (configurable behavior via `is_tab_multi` up to `tab_max`).

Note that opening an alias to folder replaces the original path in your url bar unlike with symlinks:
  - `symlink = /original/path/symlink/path`
  - `alias   =               /aliased/path`


To customize the plugin behavior, paste the following configuration lines to you `behavior { actions { ⎀ }}` section and change them; the built-in [config editor](https://marta.sh/docs/configuration/editor/)[^1] contains helpful tips on what each option does and what values it accepts
<details>
  <summary>Click to expand config</summary>

```
  "es¦🗁.alias.is_tab_multi"	true
  "es¦🗁.alias.tab_max"     	6
```

</details>

To use the plugin add a key binding to `keyBindings { ⎀ }`
  - `";"     "es¦🗁.open⤻alias"` using the plugin default or user configured values
  - `"o" {id "es¦🗁.open⤻alias" is_tab_multi false tab_max 2}` using per-command custom values

['Open alias' plugin file](https://github.com/eugenesvk/Marta.ext/blob/main/Plugins/es¦🗁open⤻.lua)

## Theme

![Pane colors](<./img/Marta Pane.png>)

[Theme file](https://github.com/eugenesvk/Marta.ext/blob/main/Themes/esWhite.theme)

## Keybinds
TBD

## Installation

__Plugins__: copy files from the [Plugins](https://github.com/eugenesvk/Marta.ext/blob/main/Plugins) folder to the same folder in your config folder[^3]

__Theme__: copy files from the [Themes](https://github.com/eugenesvk/Marta.ext/blob/main/Themes) folder to the same folder in your config folder[^3], then

  - either run the `Switch Theme` action and select `esWhite`
  - or set `theme "esWhite"` in the `behavior {}` section of your `conf.marco` file


__Keybinds__: copy the `keyBindings` section from the [conf.marco](https://github.com/eugenesvk/Marta.ext/blob/main/conf.marco) file to the same file in your config folder[^3]

[^1]: open via `Preferences` action
[^2]: in the [Actions Panel](https://marta.sh/docs/core/actions/#actions-panel)
[^3]: _~/Library/Application Support/org.yanex.marta_ by default, open via `Open Configuration Folder` action

## Known issues

[Link](#textcolorbluetextrm1textcolorgreytextrm-link)

  - Due to a lack of Marta alias/hardlink creation APIs (upvote the corresponding issues [alias](https://github.com/marta-file-manager/marta-issues/issues/351), [hardlink](https://github.com/marta-file-manager/marta-issues/issues/981)) you should install [alisma](https://eclecticlight.co/taccy-signet-precize-alifix-utiutility-alisma/) to create aliases (hardlinks are using the system `ln`)
    - [alisma 3a (Universal binary for El Capitan to Monterey)](https://eclecticlightdotcom.files.wordpress.com/2022/02/alisma3a.zip) or 
    - [alisma 2 (Intel-only for El Capitan to Catalina)](https://eclecticlightdotcom.files.wordpress.com/2019/06/alisma2.zip)
  - Configuration tips don't display unicode values properly, [see this bug](https://github.com/marta-file-manager/marta-issues/issues/975 )
  - After a hardlink is created the file list is not refreshed like it's when a symlink/alias is created, refresh manually to see the new file
  - Action names have the default icons even if the user specified custom ones, can't load user config on plugin load (see this [issue](https://github.com/marta-file-manager/marta-issues/issues/983)). Workaround: manually replace the icons in the `marta.action(` lines of your copy of the ['Link' plugin file](https://github.com/eugenesvk/Marta.ext/blob/main/Plugins/es¦file_link.lua)

[Tab deduplication](#textcolorbluetextrm2textcolorgreytextrm-tab-deduplication)

  - Closing a tab with a mouse doesn't trigger anything; it also doesn't activate the left tab (see [this issue](https://github.com/marta-file-manager/marta-issues/issues/969))

[Open alias](#textcolorbluetextrm3textcolorgreytextrm-open-alias)

  - When opening multiple folders in new tabs the displayed content might be from a different tab, switching back and forth should fix it

## Credits

[@yanex](https://github.com/yanex) for creating this highly customizable native macOS file manager;
[@page-down](https://github.com/page-down) for his help with the _Tab deduplication_ plugin

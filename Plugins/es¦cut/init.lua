plugID = "es¦cut"
marta.plugin({id=plugID,name="Cut command via a Rust plugin",apiVersion="2.2"})

local ctxG 	= marta.globalContext	--
local cfgP 	= ctxG.application.configurationFolder.rawValue
local plugP	= ctxG.application.pluginFolder -- .rawValue BUG github.com/marta-file-manager/marta-issues/issues/1089

package.cpath = package.cpath ..';'.. plugP..'/lib/?.dylib'
local libes_rs	= require "libes_rs" -- name = `#[mlua::lua_module(name="es_rs")]` annotated fn at es_rs.lib.rs

marta.action({id="📋cut",name="Cut the currently selected items to the 📋clipboard ()"  ,
  isApplicable = function(ctxA) return ctxA.activePane.model.hasActiveFiles end,
  apply        = function(ctxA) clipboard_cut_paste({ctxA=ctxA,op="cut"})  ; end})
marta.action({id="📋paste.move.cut",name="Move the previously ‘📋cut’ items to the currently active tab or paste copied items"  ,
  isApplicable = function(ctxA) return ctxA.activePane.model.hasActiveFiles end,
  apply        = function(ctxA) clipboard_cut_paste({ctxA=ctxA,op="move"})  ; end})
marta.action({id="📋paste.move.any",name="Paste&move any items from the 📋clipboard to the currently active tab (even if not added via ‘📋cut’, but ‘core.clipboard.copy’)"  ,
  isApplicable = function(ctxA) return ctxA.activePane.model.hasActiveFiles end,
  apply        = function(ctxA) clipboard_cut_paste({ctxA=ctxA,op="move.any"})  ; end})

local _d = 1

is_cut = false
function clipboard_cut_paste(arg)
  local ctxA    	= arg.ctxA	-- holds refs to PaneContext instances for active+inactive panes
  local ctxW    	= ctxA.window
  local ctxG    	= marta.globalContext	--
  local actG    	= ctxG.actions
  local fsL     	= marta.localFileSystem	--
  local ctxPA   	= ctxA.activePane      	--
  local ctxP_inA	= ctxA.inactivePane    	--
  local modelA  	= ctxPA.model          	-- Active pane list model
  local viewP   	= ctxPA.view           	--

  function pss(msg) viewP:showNotification(msg,plugID,"short") end -- short-term "print" to the statusbar
  function psl(msg) viewP:showNotification(msg,plugID,"long" ) end -- long-term  "print" to the statusbar
  function dbg(l,msg) if (_d>=l) then pss(msg) end end -- short-term "print" to the statusbar
  function run_action(action) ctxW:runAction(actG:getById(action),ctxPA) end -- short-term "print" to the statusbar

  -- local cb_alert = function(name) if name then martax.alert("Hello, " .. name .. "!") end end
  cb_alert = function(name) martax.alert(name) end
  local cb_res = libes_rs.ask_overwrite(ctxA, cb_alert)
  -- psl("cb_res = "..cb_res)
  -- psl(type(ctxW.nsWindow)) -- userdata
  -- psl(type(cb_alert))


  -- local is_fs = modelA.isLocalFileSystem -- ignore non-folders, e.g., zip files
  -- if not is_fs then dbg(1,plugID.."📋 can't run in a non-local filesystem"); return; end
  -- local is_dir = false
  -- local parentFdInf = modelA.folderInfo
  -- local path_to
  -- if   parentFdInf then
  --   path_to = parentFdInf.path.rawValue
  --   if parentFdInf.isFolder then is_dir = true end
  -- end
  -- local can_paste = is_fs and is_dir
  -- if _d>=4 then pss("can_paste="..tostring(can_paste).." local_fs="..tostring(is_fs).." dir="..tostring(is_dir)) end
  -- local cb_res = libes_rs.move_to(ctxA, path_to)
  -- psl("cb_res = "..cb_res)

  -- psl(type(ctxA.args))

  -- local errt = libes_rs.errort()
  -- psl("errt = "..errt)

  -- set our global flag to true to track ↓ our cut, but otherwise ↓ use native copy
  -- if     (arg.op == "cut"     ) then is_cut = true ; run_action("core.clipboard.copy"); pss('📋cut')
  -- elseif (arg.op == "move"    ) then if not can_paste then pss("❗not a 📁, can't paste here"); return end
  --   if   (is_cut == true      ) then is_cut = false; libes_rs.move_to(path_to)        ; dbg(2,'‘📋paste.move.cut’')
  --   else run_action("core.clipboard.paste");             dbg(2,"No items were ‘📋cut’, pasting copied…")
  --   end
  -- elseif (arg.op == "move.any") then if not can_paste then pss("❗not a 📁, can't paste here"); return end
  --   if   (is_cut == true      ) then is_cut = false else dbg(2,"No items were ‘📋cut’, moving copied…") end
  --   libes_rs.move_to(path_to)
  -- end
end

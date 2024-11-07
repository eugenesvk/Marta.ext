local plugID = "es¦cut"
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
  local model   	= ctxPA.model          	-- Active pane list model
  local viewP   	= ctxPA.view           	--

  function pss(msg) viewP:showNotification(msg,plugID,"short") end -- short-term "print" to the statusbar
  function psl(msg) viewP:showNotification(msg,plugID,"long" ) end -- long-term  "print" to the statusbar
  function dbg(l,msg) if (_d>=l) then pss(msg) end end -- short-term "print" to the statusbar

  function run_action(action) ctxW:runAction(actG:getById(action),ctxPA) end -- short-term "print" to the statusbar

  function pss(msg) viewP:showNotification(msg,plugID,"short") end -- short-term "print" to the statusbar
  -- set our global flag to true to track our cut ↓, but otherwise ↓ use native copy libes_rs.cut()
  if     (arg.op == "cut"     ) then is_cut = true ; run_action("core.clipboard.copy"); pss('📋cut')
  elseif (arg.op == "move"    ) then
    if   (is_cut == true      ) then is_cut = false; libes_rs.move()                  ; dbg(1,'‘📋paste.move.cut’')
    else run_action("core.clipboard.paste");             dbg(1,"No items were ‘📋cut’, pasting copied…")
    end
  elseif (arg.op == "move.any") then
    if   (is_cut == true      ) then is_cut = false else dbg(1,"No items were ‘📋cut’, moving copied…") end
    libes_rs.move()
  end
end

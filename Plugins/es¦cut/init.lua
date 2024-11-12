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
marta.action({id="📋paste.move.cut",name="Paste&move the previously ‘📋cut’ items to the currently active tab (ignore items added directly via ‘core.clipboard.copy’)"  ,
  isApplicable = function(ctxA) return ctxA.activePane.model.hasActiveFiles end,
  apply        = function(ctxA) clipboard_cut_paste({ctxA=ctxA,op="move"})  ; end})
marta.action({id="📋paste.move.any",name="Paste&move any items from the 📋clipboard to the currently active tab (even if not added via ‘📋cut’, but ‘core.clipboard.copy’)"  ,
  isApplicable = function(ctxA) return ctxA.activePane.model.hasActiveFiles end,
  apply        = function(ctxA) clipboard_cut_paste({ctxA=ctxA,op="move.any"})  ; end})

is_cut = false
function clipboard_cut_paste(arg)
  local ctxA    	= arg.ctxA             	-- holds refs to PaneContext instances for active+inactive panes
  local ctxG    	= marta.globalContext  	--
  local fsL     	= marta.localFileSystem	--
  local ctxPA   	= ctxA.activePane      	--
  local ctxP_inA	= ctxA.inactivePane    	--
  local model   	= ctxPA.model          	-- Active pane list model
  local viewP   	= ctxPA.view           	--
  local filesInf	= model.activeFileInfos	-- array of FileInfo with all the attributes, gathered on folder load (so cached) (ZIP fs doesn't store macOS extended attributes)
  local parentFd	= model.folder         	--
  if     (arg.op == "cut"     ) then is_cut = true ; libes_rs.cut     () -- set our global flag to true to track our cut
  elseif (arg.op == "move"    ) then is_cut = false; libes_rs.move    ()
  elseif (arg.op == "move.any") then is_cut = false; libes_rs.move_any()
  end

end

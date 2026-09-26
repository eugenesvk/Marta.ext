local plugID = "es¦🗁" -- 📂🖿🗀🗁🮹
marta.expose()
marta.plugin({id=plugID,name="Folder actions (incl. aliases⤻folders)",apiVersion="2.2"})

local ctxG 	= marta.globalContext	--
local cfgP 	= ctxG.application.configurationFolder.rawValue
local plugP	= ctxG.application.pluginFolder -- .rawValue BUG github.com/marta-file-manager/marta-issues/issues/true089

marta.action({id="open⤻alias",name="Open, incl. alias⤻🗁",shortName="⤻Alias",menuName="Open+alias⤻🗁",
  isApplicable = function(ctxA) return ctxA.activePane.model.hasActiveFiles end,
  apply        = function(ctxA) open_alias({ctxA=ctxA,is_tab_multi,tab_max})  ; end})

local cfgID = "alias"
local cfgPP = plugID ..'.'.. cfgID ..'.' -- config path prefix
marta.configurationKey("behavior","actions",cfgPP .. "is_tab_multi",{typeConstraints={"boolean"},examples={"t̳r̳u̳e̳","false"},
  description = "Open 2nd+ (alias⤻) folder in a new tab (d̳e̳f̳, up to 'tab_max')",})
marta.configurationKey("behavior","actions",cfgPP .. "tab_max"     ,{typeConstraints={"int"    },examples={"6̳"   ,"1"    },
  description = "When multiple (aliases⤻) folders selected, open up to this many in new tabs (d̳e̳f̳, inclusive)",})

local _d = 0

function open_alias(arg)
  local ctxA 	= arg.ctxA	-- holds refs to PaneContext instances for active+inactive panes
  local ctxW 	= ctxA.window
  local ctxG 	= marta.globalContext
  local actG 	= ctxG.actions
  local arg_u	= ctxA.args	-- arguments passed from users when invoking the action

  local ctxPA   	= ctxA.activePane       	-- ctxA holds refs to PaneContext instances for active+inactive panes
  local list_m  	= ctxPA.model           	-- Active pane list model
  local files   	= list_m.activeFiles    	-- array of File objects which are bare pointers to files, no attributes (not all fs store them, ZIP doesn't store macOS extended attributes)
  local filesInf	= list_m.activeFileInfos	-- array of FileInfo with all the attributes, gathered on folder load, so cached (ZIP fs doesn’t store macOS extended attributes, so isApplication will always return false)
  local viewP   	= ctxPA.view

  function pss(msg) viewP:showNotification(msg,plugID,"short") end -- short-term "print" → statusbar
  function psl(msg) viewP:showNotification(msg,plugID,"long" ) end -- long -term "print" → statusbar
  function dbg(l,msg) if (_d>=l) then pss(msg) end end -- short-term "print" to the statusbar if dbg≥level
  function _d1(  msg) if (_d>=1) then pss(msg) end end
  function _d2(  msg) if (_d>=2) then pss(msg) end end
  function _d3(  msg) if (_d>=3) then pss(msg) end end

  -- Get and validate user configuration values
  local cfgDef,cfgPath,cfgBeh,cfgAct, cfgTMulti,cfgTMax, is_tab_multi,tab_max

  cfgDef      	 = {["is_tab_multi"]=true,["tab_max"]=6,}
  cfgAct      	 = ctxG:get("behavior","actions") -- crashes without the extra path element
  if cfgAct   	~= nil then
    cfgTMulti 	 = cfgAct[cfgPP .. "is_tab_multi"  ]
    cfgTMax   	 = cfgAct[cfgPP .. "tab_max"    ] end
  -- function argument → config → ≝
  if is_tab_multi	==nil then is_tab_multi	=arg_u.is_tab_multi    	end
  if is_tab_multi	==nil then is_tab_multi	=cfgTMulti             	end
  if is_tab_multi	==nil then is_tab_multi	=cfgDef['is_tab_multi']	end
  if tab_max     	==nil then tab_max     	=arg_u.tab_max         	end
  if tab_max     	==nil then tab_max     	=cfgTMax               	end
  if tab_max     	==nil then tab_max     	=cfgDef['tab_max']     	end
  local _e = "❗open⤻alias: wrong ‘"
  local _s = " config, using ≝'"
  if (type(is_tab_multi	) ~= "boolean"            	)                               	--
    then   is_tab_multi	   = cfgDef['is_tab_multi'	];pss(_e..cfgPP.."is_tab_multi’"	.._s..is_tab_multi.."’") end
  if (type(tab_max     	) ~= "number"             	)                               	--
    or (not            	isint(tab_max             	)                               	--
    or (   tab_max     	< 0)                      	)                               	--
    then   tab_max     	   = cfgDef['tab_max'     	];pss(_e..cfgPP.."tab_max’"     	.._s..tab_max.."’" ) end
  if _d>=2 then psl("res= " ..tostring(is_tab_multi).. " UsrArg=" .. tostring(arg_u.is_tab_multi) .. " UsrCfg=" .. tostring(cfgTMulti) .. " ≝" .. tostring(cfgDef['is_tab_multi'])) end
  if _d>=3 then martax.alert("Config vs Validated",(tostring(cfgTMulti) or '✗') ..'|'.. (tostring(cfgTMax) or '✗')) end


  -- Define helper actions
  function run_action (action ) ctxW:runAction(actG:getById(action),ctxPA) end -- run marta actions
  function dbg_action (action )
    martax.alert("?=" ..action);ctxW:runAction(actG:getById(action),ctxPA) end
  function run_act_ctx(act,ctx) ctxW:runAction(actG:getById(act),ctx) end -- run marta actions with custom context
  function open_tab   (file   ) ctxW.tabs:open(file,nil,ctxPA) end --nil=nameToSelect?
    -- martax.alert("opening file in a new tab = " ..file.path.rawValue)
  function open_dir   (file   ) -- create custom context to pass to Marta's open folder action to use the current tab
    -- martax.alert("opening dir in the current tab = " ..file.path.rawValue)
    local my_ctxA = {window=ctxW, toWeak = function() ctxA:toWeak() end,
      activePane  =ctxA.  activePane,
      inactivePane=ctxA.inactivePane,
      args = {src = file.path.rawValue}, -- Dictionary<String, Any>
    }
    run_act_ctx("core.open.folder",my_ctxA)
  end

  if #filesInf == 0 then _d1("open⤻alias: no selection") return end -- skip an empty dir

  local err_m = ""
  local dir_count = 0
  for i, tgtFI in ipairs(filesInf) do -- Iterate thru active=(selected¦cursor) files
    local file  	= tgtFI.file
    local name  	= tgtFI.name
    local path  	= tgtFI.path
    local is_dir	= tgtFI.isFolder -- open in a new tab since you can't open many dirs in Marta unlike files that call external apps
    local passthru = true -- use the builtin open command

    if tgtFI.isAlias then
      local err, target = tgtFI.file:canonicalize()
      if    err then err_m = err_m .." ✗canonicalize=".. target.path.rawValue end
      if target and     target:exists() then
        local _, info = target:readInfo("node")
        if info then
          path = info.path
          file = info.file
          if info.isFolder then is_dir = true; passthru = false; end
        end
      else end
    end

    if is_dir then
      dir_count = dir_count + 1
      if  is_tab_multi             	  then
        if     (dir_count > tab_max	) then -- don't open dirs above max count
        elseif (dir_count > 1      	) then open_tab  (file)
        else                       	       open_dir  (file) end	--
      elseif   (dir_count > 1      	) then                     	-- ignore mutiple dirs without multitab option
      else                         	       open_dir  (file) end	-- open     1 dir
    else                           	       martax.openFiles(file.path.rawValue) end -- open not a dir
    -- core.open/core.open.folder.tab  operate on ALL selected items, so will dupe effort and replicate the original bug
    -- similarly, core.navigate.original/….tab requires a custom context, at which point we have alias resolved and can use core.open.folder
  end
  if err_m ~= "" then psl("✗" ..plugID.."open⤻alias: " .. err_m) end
end

function isempty(s) return s == nil or s == '' end
function isint  (n) return n == math.floor(n)  end

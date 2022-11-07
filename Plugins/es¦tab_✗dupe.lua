-- 2do: remove my fix (activate our original active tab) when this bug is fixed https://github.com/marta-file-manager/marta-issues/issues/969
-- "Cmd+W"	"es¦tab.✗tab_n_dupe"
  -- to ✗close the current tab, select 1 tab to the ←, ✗close all duplicate tabs in the active view
  -- current tab has priority over tabs with lower index (to the left)
marta.expose()
marta.plugin({id="es¦tab", name="Close Duplicated Tabs", apiVersion="2.1"})

marta.action({id="✗tab_n_dupe", name="Tab: ✗Close Current & Duplicates",
  apply = function(ctxA) tabClose(ctxA); tabLeft(ctxA); tabCloseDupe({ctxA=ctxA,saveCur=true}); end})
marta.action({id="✗dupe" , name="Tab: ✗Close Duplicates"  , apply = function(ctxA) tabCloseDupe({ctxA=ctxA,saveCur=true}); end})
-- marta.action({id="✗tab_cur←" , name="Tab: ✗Close Current and switch to the ←Left",
--   apply = function(ctxA) tabClose(ctxA); tabLeft(ctxA); end})
-- marta.action({id="✗tab_cur"  , name="Tab: ✗Close Current"     , apply = function(ctxA) tabClose    (ctxA); end})
-- marta.action({id="tab←"      , name="Tab: Switch to the ←Left", apply = function(ctxA) tabLeft     (ctxA); end})

function tabLeft(ctxA)
  -- martax.alert("@tabLeft")
  local ctxW   	= ctxA.window
  local tabMan 	= ctxW.tabs
  local paneMan	= ctxW.panes
  local tabA   	= paneMan.activePane -- get uptodate active tab
  local tabPos 	= tabMan:getPosition      (tabA)  --(tab: PaneContext): Option<TabPosition> -- Get the tab position
  local tabActI	= tabMan:getActiveTabIndex(tabPos) -- 0-based active tab index
  local tabLeft	= tabMan:getTab           (tabPos, tabActI-1)
  if (tabActI > 0) then -- this might not be needed since left-most tab receiving -1 command doesn't error
    tabMan:activate(tabLeft)
  end
end

function tabClose(ctxA)
  -- martax.alert("@tabClose")
  local ctxW   	= ctxA.window
  local tabMan 	= ctxW.tabs
  local paneMan	= ctxW.panes
  local tabA   	= paneMan.activePane
  -- martax.alert("@tabClose Active tab's ID=\n" .. tabA_id .. "\nwith path@=" .. tabA_path)
  tabMan:close(tabA)
end

function tabCloseDupe(arg)
  local saveCur	= arg.saveCur or false -- if current tab is a dupe, close the other dupe

  local ctxA   	= arg.ctxA
  local ctxW   	= ctxA.window
  local tabMan 	= ctxW.tabs
  local paneMan	= ctxW.panes

  local tabA     	= paneMan.activePane
  local tabPos   	= tabMan:getPosition(tabA)     -- Get the tab position          	--(tab:PaneContext):Option<TabPosition>
  local tabCount 	= tabMan:getCount   (tabPos  ) -- tab count for a given position	--(pos:             Option<TabPosition>):Int
  local tabA_id  	= tabA.id -- tab unique id
  local tabA_path	= tabA.model.folder.path       -- tab unique id
  local t        	= {}
  local i        	= 0
  if saveCur then t[tabA_path]={true,tabA_id} end -- save current tab's path/ID to not close it later
  -- martax.alert("Active tab's ID=\n" .. tabA_id .. "\nwith path@=" .. tabA_path)

  while (i < tabCount) do
    local tab   	 = tabMan:getTab(tabPos, i)
    local tab_id	 = tab.id
    local path  	 = tab.model.folder.path
    if (t[path] 	~= nil) then
      if saveCur and ( tab_id == tabA_id) then -- don't close if current tab is our original active tab
        -- martax.alert("Saving active tab ID=\n" .. tabA_id .. "\npath=" .. tabA_path)
        i	= i + 1
      else
        -- martax.alert("Closing tab with ID=\n" .. tab_id)
        tabMan:close(tab)
        tabCount	 = tabCount - 1
      end
    else
      t[path]	= {true,tab_id} -- path has been seen @ ID (use it later to close i instead of my active tab)
      i      	= i + 1
    end
  end
  if saveCur then -- activate our original active tab as focus shifts on tab close (bug)
    tabMan:activate(tabA)
  end
end

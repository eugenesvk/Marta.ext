marta.expose()
marta.plugin({id="es¦tab", name="Close Duplicated Tabs", apiVersion="2.2"})

marta.action({id="✗tab_n_dupe", name="Tab: ✗Close Current & Duplicates",
  apply = function(ctxA) tabClose(ctxA); tabLeft(ctxA); tabCloseDupe({ctxA=ctxA,saveCur=true}); end})
marta.action({id="✗dupe" , name="Tab: ✗Close Duplicates"  , apply = function(ctxA) tabCloseDupe({ctxA=ctxA,saveCur=true}); end})
-- marta.action({id="✗tab_cur←" , name="Tab: ✗Close Current and switch to the ←Left",
--   apply = function(ctxA) tabClose(ctxA); tabLeft(ctxA); end})

function tabLeft(ctxA)
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
  local ctxW   	= ctxA.window
  local tabMan 	= ctxW.tabs
  local paneMan	= ctxW.panes
  local tabA   	= paneMan.activePane
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
  local tabA_path	= tabA.model.folder.path       -- tab unique Path (≠String, use rawValue for that)
  local t        	= {}
  local i        	= 0
  if saveCur then t[tabA_path.rawValue]={true,tabA.id} end -- save current tab's path/ID to not close it later

  while (i < tabCount) do
    local tab    	 = tabMan:getTab(tabPos, i)
    local path_s 	 = tab.model.folder.path.rawValue
    if (t[path_s]	~= nil) then
      if saveCur and (tab.id == tabA.id) then -- don't close if current tab is our original active tab
        i	= i + 1
      else
        tabMan:close(tab)
        tabCount	 = tabCount - 1 -- reuse index (it can be a ←moved tab), decrease the # of iters from the top
      end
    else
      t[path_s]	= {true,tab.id} -- path has been seen @ ID (use it later to close i instead of my active tab)
      i        	= i + 1
    end
  end
  if saveCur then -- activate our original active tab as focus shifts on tab close (bug)
    tabMan:activate(tabA)
  end
end

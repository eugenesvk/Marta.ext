-- marta.expose()
-- marta.plugin({id="bug", name="path comparison bug", apiVersion="2.1"})

-- marta.action({id="id_tab_pathcomp21", name="Tab: Path comparison bug 2.1"
--   ,apply=function(ctxA) tabPathComparison({ctxA=ctxA}); end})

-- local std = {} -- stdlib
-- -- Print contents of `tbl`, with indentation `indent`, to string
-- function std.t2str(tbl, indent, out)
--   if not indent then indent = 0  end
--   if not out    then out    = '' end
--   for k, v in pairs(tbl) do
--     key = string.rep("  ",indent)..k..": "
--     if     type(v) == "table"   then	out = out..key             ..'\n'; out=std.t2str(v,indent+1,out)
--     elseif type(v) == "boolean" then	out = out..key..tostring(v)..'\n'
--     elseif type(v) == "string"  then	out = out..key..         v ..'\n'
--     else                            	out = out..key..tostring(v)..'\n' end
--   end
--   return out
-- end

-- function tabPathComparison(arg)
--   local ctxA   	= arg.ctxA
--   local ctxW   	= ctxA.window
--   local tabMan 	= ctxW.tabs
--   local paneMan	= ctxW.panes

--   local tabA     	= paneMan.activePane
--   local tabPos   	= tabMan:getPosition(tabA)     -- Get the tab position          	--(tab:PaneContext):Option<TabPosition>
--   local tabCount 	= tabMan:getCount   (tabPos  ) -- tab count for a given position	--(pos:             Option<TabPosition>):Int
--   local i        	= 0

--   print("tabCount = "..tabCount)
--   local prev = nil
--   local dupe = false
--   local path_list = {}
--   while (i < tabCount) do
--     local tab    	 = tabMan:getTab(tabPos, i)
--     local path   	 = tab.model.folder.path --FAILS
--     -- local path	 = tab.model.folder.path.rawValue --WORKS
--     if (path == prev) then
--       martax.alert("DUPE path" .. tostring(i).. tostring(path))
--       dupe = true
--     end
--     path_list[i] = path
--     prev = path
--     i	= i + 1
--   end
--   if not dupe then
--     martax.alert("NO dupes in paths" .. std.t2str(path_list) .."\n"..prev.__tostring())
--   end
-- end

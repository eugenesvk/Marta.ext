marta.expose()
local plugID = "es¦test"
marta.plugin({id=plugID, name="Tests", apiVersion="2.2"})

marta.action({id="notify"         	, name="test notify1", apply = function(ctxA) tst_notify  (ctxA); end})
marta.action({id="info_dir_change"	, name="test notify2", apply = function(ctxA) t_dir_change(ctxA); end})

function tst_notify(ctxA)
  -- martax.alert("@test")
  local ctxW   	= ctxA.window
  local tabMan 	= ctxW.tabs
  local paneMan	= ctxW.panes
  local tabA   	= paneMan.activePane -- get uptodate active tab

  local ctxP 	= ctxA.activePane
  local viewP	= ctxP.view
  viewP:showNotification("notification_text2show","id1","short")
    -- text    	: String
    -- id      	: String? = nil  -- all msgs in the queue with this id will be removed
    -- duration	: Option<NotificationDuration> = "short"/"long"
  viewP:addStatusText("status_text_2show", "status_id_1", "middle")
    -- StatusTextPosition  beginning ending exclusive middle
  -- viewP:removeStatusText("status_id_1")
end

-- listModelHandler({ -- hook into location changes
--   locationChanged = function(context)
--     local parent = context.activePane.model.folder
--     if not parent then return end

--     local pluginId = 'aaa'
--     local text = "myLocation changed: " .. parent.path
--     context.activePane.view:showNotification(text, pluginId .. ".notification")
--   end
-- }
-- listModelHandler({ -- hook into location changes
--   locationChanged = function(context)
--     -- Load display preferences from .DS_Store-like file in current dirctory
--     -- if exists, apply it...
--     -- context.activePane.model.ordering = marta.ListOrdering {
--     --   associatedColumn  	= "Modified",
--     --   isAscending       	= false,
--     --   isDirectoriesOnTop	= false,
--     --   compare = function(first, second)
--     --     return first.dateModified < second.dateModified
--     --   end
--     -- }
--   end
-- })

function t_dir_change(ctxA)
  -- Save current display state to .DS_Store-like file in current directory
end

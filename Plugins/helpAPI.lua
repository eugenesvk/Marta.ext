marta.expose()
local plugID = "helpAPI"
marta.plugin({id=plugID, name="Various API helper functions", apiVersion="2.2"})

local ctxG 	= marta.globalContext	--
local cfgP 	= ctxG.application.configurationFolder.rawValue
local plugP	= ctxG.application.pluginFolder -- .rawValue BUG github.com/marta-file-manager/marta-issues/issues/1089

marta.action({id="id_helper", name="Test",apply=function(ctxA) helpTab({ctxA=ctxA}); end})

function helpTab(arg)
  local ctxA      	= arg.ctxA	-- holds refs to PaneContext instances for active+inactive panes
  local ctxW      	= ctxA.window
  local ctxG      	= marta.globalContext	--
  local actG      	= ctxG.actions -- class marta.Actions {
    -- actions    	         : Array<Action> All available actions
    --𝑓 getById   	(actionId) :       Action? Returns the action with the given identifier, get id by holding ⎇ in ⌘e=core.actions panel
    --𝑓 getHotkey 	(actionId) :       Hotkey? 1st hotkey for the action with such id
    --𝑓 getHotkeys	(actionId) : Array<Hotkey> all hotkeys
  local fsL       	= marta.localFileSystem	--
  local ctxPA     	= ctxA.activePane      	--
  local ctxP_inA  	= ctxA.inactivePane    	--
  local model     	= ctxPA.model          	-- Active pane list model
  local viewP     	= ctxPA.view           	--

  -- "Aliases"
  function run_action(action) ctxW:runAction(actG:getById(action),ctxPA) end -- short-term "print" to the statusbar
    -- 𝑓 WindowContext:runAction(
    -- action 	: Action,
    -- context	: [PaneContext | ActionContext]? ≝ nil → starts in active pane

  -- Notification
  function pss(msg) viewP:showNotification(msg,plugID,"short") end -- short-term "print" to the statusbar
  function psl(msg) viewP:showNotification(msg,plugID,"long" ) end -- long-term  "print" to the statusbar
    -- 𝑓 PaneView:showNotification(
    text    	= "String"                      	--
    id      	= "String?"                     	--≝nil all same-id messages are removed
    duration	= "Option<NotificationDuration>"	--≝short¦long (enum marta.NotificationDuration)

  local tabMan 	= ctxW.tabs
  local paneMan	= ctxW.panes

  local tabA    	= paneMan.activePane
  local tabPos  	= tabMan:getPosition(tabA)     -- Get the tab position          	--(tab:PaneContext):Option<TabPosition>
  local tabCount	= tabMan:getCount   (tabPos  ) -- tab count for a given position	--(pos:             Option<TabPosition>):Int
  local i       	= 0
end

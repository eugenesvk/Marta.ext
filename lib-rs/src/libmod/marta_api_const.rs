// Associated constant with a struct for autocomplete and typo avoidance
pub trait C_ActionContext {
  const activePane  	:&str ="activePane"  	;
  const args        	:&str ="args"        	;
  const inactivePane	:&str ="inactivePane"	;
  const window      	:&str ="window"      	;
  const toWeak      	:&str ="toWeak"      	;
}
pub trait C_WindowContext { //marta.sh/api/marta/windowcontext.type/
  const isEnabled	:&str = "isEnabled"	;//let : Boolean
  const nsWindow 	:&str = "nsWindow" 	;//let : LightUserData<NSWindow>
  const panes    	:&str = "panes"    	;//let : PaneManager
  const tabs     	:&str = "tabs"     	;//let : TabManager
  const runAction	:&str = "runAction"	;//fun (action, context)
}
pub trait C_PaneContext {
  const id           	:&str ="id"           	;//let: String
  const model        	:&str ="model"        	;//let: ListModel
  const tabIndex     	:&str ="tabIndex"     	;//let: Int
  const view         	:&str ="view"         	;//let: PaneView
  const windowContext	:&str ="windowContext"	;//let: WindowContext
  const activateTab  	:&str ="activateTab"  	;//fun()
}
pub trait C_PaneView { //marta.sh/api/marta/paneview.type/
  const currentItemRect         	:&str	="currentItemRect"         	;//let : Rect
  const addStatusText           	:&str	="addStatusText"           	;//fun (text, id, position)
  const ensureCurrentItemVisible	:&str	="ensureCurrentItemVisible"	;//fun ()
  const removeStatusText        	:&str	="removeStatusText"        	;//fun (id)
  const showNotification        	:&str	="showNotification"        	;//fun (text, id, duration)
}
pub struct ctxA;
pub struct ctxP;
pub struct ctxW;
pub struct viewP;
impl C_ActionContext	for ctxA 	{}
impl C_WindowContext	for ctxW 	{}
impl C_PaneContext  	for ctxP 	{}
impl C_PaneView     	for viewP	{}

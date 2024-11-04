marta.expose()
local plugID = "es¦test2"
marta.plugin({id=plugID, name="Tests", apiVersion="2.2"})

marta.action({id="test2"	, name="test notify2", apply = function(ctxA) test2  (ctxA); end})
-- local ctxG           	= marta.globalContext
-- local cfgBeh         	= ctxG.get("behavior","actions")

function test2(ctxA)

end

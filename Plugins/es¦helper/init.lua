-- 2DO: replace uuid with martax.uuid
--[[install deps @ init.lua location:
luarocks install --tree rocks uuid
luarocks install --tree rocks lpeg
--]]
--marta.plugin({id="es¦helper",name="Marta helper functions",apiVersion="2.1"})

marta.useRocks()
local uuid = require("uuid")
local lpeg = require("lpeg")

lpeg.locale (lpeg)
local alpha,alnum,cntrl,digit,graph,lower,punct,space,uppe,xdigit = lpeg.alpha,lpeg.alnum,lpeg.cntrl,lpeg.digit,lpeg.graph,lpeg.lower,lpeg.punct,lpeg.space,lpeg.upper,lpeg.xdigit
local P,V,Cg,Ct,Cc,S,R,C,Cf,Cb,Cs = lpeg.P,lpeg.V,lpeg.Cg,lpeg.Ct,lpeg.Cc,lpeg.S,lpeg.R,lpeg.C,lpeg.Cf,lpeg.Cb,lpeg.Cs
function string:pgsub(patt, repl) -- ~string.gsub. get pattern / replacement, substitute the replacement value for all occurrences of the pattern in a given string:
  patt = P(  patt)
  patt = Cs((patt / repl + 1)^0)
  return lpeg.match(patt, self)
end

marta.action({id="uuid"     ,name="Get u-u-i-d"    	, apply = function(ctxA) getUUID({ctxA=ctxA}) end})
marta.action({id="uuid_qd"  ,name="Get \"u-u-i-d\""	, apply = function(ctxA) getUUID({ctxA=ctxA,q='"'}) end})
marta.action({id="uuid_qs"  ,name="Get 'u-u-i-d'"  	, apply = function(ctxA) getUUID({ctxA=ctxA,q="'"}) end})
marta.action({id="uuid_s"   ,name="Get uuid"       	, apply = function(ctxA) getUUID({ctxA=ctxA,strip=true}) end})
marta.action({id="uuid_s_qd",name="Get \"uuid\""   	, apply = function(ctxA) getUUID({ctxA=ctxA,strip=true,q='"'}) end})
marta.action({id="uuid_s_qs",name="Get 'uuid'"     	, apply = function(ctxA) getUUID({ctxA=ctxA,strip=true,q="'"}) end})

function getUUID(arg)
  local q    	= arg.q     or ''
  local strip	= arg.strip or false
  local uuid 	= uuid()
  if strip then uuid = uuid:pgsub("-","") end
  martax.clipboard.setString(q .. uuid .. q)
end

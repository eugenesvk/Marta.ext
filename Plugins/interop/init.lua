-- Plugin loading a rust library
local plugID = "es¦rs"
marta.plugin({id=plugID,name="Marta example of a compiled interop plugin (Rust)",apiVersion="2.2"})

local ctxG 	= marta.globalContext	--
local cfgP 	= ctxG.application.configurationFolder.rawValue
local plugP	= ctxG.application.pluginFolder -- .rawValue BUG github.com/marta-file-manager/marta-issues/issues/1089

package.cpath = package.cpath ..';'.. plugP..'/lib/?.dylib'
local librustinterop	= require "libes_rs" -- name = `#[mlua::lua_module(name="es_rs")]` annotated fn at es_rs.lib.rs

-- librustinterop.lua_plugin_within_rust()
marta.action({id="rust",name="test librustinterop",
  isApplicable = function(ctxA) return ctxA.activePane.model.hasActiveFiles end,
  apply        = function(ctxA) luarustinterop(ctxA) end})

mytostring = tostring
myalert = martax.alert

function luarustinterop(ctxA)
  -- martax.alert('libcinterop.used_memory()',tostring(librustinterop.used_memory()))
  -- librustinterop.console("my_console")
  -- martax.alert('libcinterop.ret_s()',tostring(librustinterop.ret_s()))
  librustinterop.call_lua_fn() --= myalert(tostring(123))

  -- a= librustinterop.hello("my_name")
  -- martax.alert('libcinterop.hello(myname)',tostring(a.name))
  -- martax.alert("librustinterop.hello('Ep')['name_in_table']()",librustinterop.hello('Ep')['name_in_table'])
  -- martax.alert('librustinterop.hello_world()'                 ,librustinterop.hello_world())
  -- librustinterop.try_api()
  -- local cfgDef	 = {["affixSym"]='🔗',["affixAlias"]='⤻',["spot"]='stem',["lnkMax"]=3,["iterMax"]=5
    -- ,         	   ["linkT"]="sym",["binAlias"]='/usr/local/bin/alisma',["binHard"]='/bin/ln',}
  -- local cfgDef = {affixSym="🔗"}
  -- martax.alert('librustinterop.parse_table("cfgDef")',librustinterop.parse_table("cfgDef"))
  -- martax.alert('librustinterop.parse_table(cfgDef)',librustinterop.parse_table(cfgDef))
  -- martax.alert('librustinterop.used_memory()',tostring(librustinterop.used_memory())) -- +
  -- martax.alert('librustinterop.sum(2+2)',tostring(librustinterop.sum(2,2))) -- +

  -- martax.alert('librustinterop.parse_ctx_a(ctxA)',librustinterop.parse_ctx_a(ctxA))
  -- librustinterop.parse_table(cfgDef)
end

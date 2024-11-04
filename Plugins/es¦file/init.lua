marta.plugin({id="es¦file",name="File information",apiVersion="2.2"})
--[[install deps @ init.lua location:
luarocks install --tree rocks uuid
luarocks install --tree rocks lpeg
--]]
marta.useRocks() -- import LuaRocks dependencies from 'rocks' or useRocks("custom_dir")
local uuid = require("uuid")
local lpeg = require("lpeg")

lpeg.locale (lpeg)  -- get digit, alpha, etc.
local alpha, cntrl, digit, graph, lower, punct, space, upper, alnum, xdigit =
  lpeg.alpha, lpeg.cntrl, lpeg.digit, lpeg.graph, lpeg.lower, lpeg.punct,
  lpeg.space, lpeg.upper, lpeg.alnum, lpeg.xdigit
-- Now, the “P” function makes a pattern according to what you supply it. The simple case is a string literal, so that P“foo” matches “foo”.
local P, V, Cg, Ct, Cc, S, R, C, Cf, Cb, Cs = -- save typing function names with "lpeg" in front of them:
  lpeg.P, lpeg.V, lpeg.Cg, lpeg.Ct, lpeg.Cc, lpeg.S, lpeg.R, lpeg.C, lpeg.Cf, lpeg.Cb, lpeg.Cs
function string:pgsub(patt, repl) -- ~string.gsub. get pattern / replacement, substitute the replacement value for all occurrences of the pattern in a given string:
  patt = P(  patt)
  patt = Cs((patt / repl + 1)^0)
  return lpeg.match(patt, self)
end

marta.action({id="info",name="test Show file information",
isApplicable = function(ctxA) return ctxA.activePane.model.hasActiveFiles end,
  -- ↑ called before running an action, so you can be considerably safe avoiding some checks in apply(). However, in a multi-threaded world, some apps might change it, so still do checks
  -- Now, if you try to run the action inside an empty folder, no popup will appear. Instead, the "Show file information" can't be run in this context. message will be shown in a status bar.
apply        = function(ctxA)
  local P, V, Cg, Ct, Cc, S, R, C, Cf, Cb, Cs = -- save typing function names with "lpeg" in front of them:
    lpeg.P, lpeg.V, lpeg.Cg, lpeg.Ct, lpeg.Cc, lpeg.S, lpeg.R, lpeg.C, lpeg.Cf, lpeg.Cb, lpeg.Cs
  -- character classes
  lpeg.locale (lpeg)  -- get digit, alpha, etc.
  local alpha, cntrl, digit, graph, lower, punct, space, upper, alnum, xdigit =
    lpeg.alpha, lpeg.cntrl, lpeg.digit, lpeg.graph, lpeg.lower, lpeg.punct,
    lpeg.space, lpeg.upper, lpeg.alnum, lpeg.xdigit
  -- Now, the “P” function makes a pattern according to what you supply it. The simple case is a string literal, so that P“foo” matches “foo”.

  -- local p = lpeg.R"az"^1 * -1 -- matches a word followed by end-of-string
  local p = R"az"^1 * -1 -- matches a word followed by end-of-string
  local res1 = p:match   (   "hello") --> 6
  local res2 = lpeg.match(p, "hello") --> 6
  local res3 = p:match   ( "1 hello") --> nil

  -- martax.alert(uuid() .. "\n" .. tostring(res1) .. tostring(res2) .. tostring(res3))

  local ctxPA   	= ctxA.activePane      	-- ctxA holds refs to PaneContext instances for active+inactive panes
  local model   	= ctxPA.model          	-- Active pane list model
  local files   	= model.activeFiles    	-- array of File objects which are bare pointers to files, no attributes (not all fs store them, ZIP doesn't store macOS extended attributes)
  local filesInf	= model.activeFileInfos	-- array of FileInfo with all the attributes we need, gathered on folder load, so cached (ZIP fs doesn’t store macOS extended attributes, so isApplication will always return false)

  if #filesInf == 0 then martax.alert("No files selected.") return end -- skip an empty dir

  local text = ""
  -- Iterate through all active files, Active files are selected files, if at least one file is selected, or a file under cursor. For an empty directory, `#activeFiles == 0`.
  for _, file in ipairs(filesInf) do
    local name = file.name
    local entity

    if file.isFolder then
      entity = "[" .. name .. "]"
    else
      local size       	= martax.formatSize(file.size)
      -- local size_fmt	= size:gsub("bytes",""):gsub("B",""):gsub("%s"," "):gsub("173","")
      -- local size_fmt	= size:gsub("bytes",""):gsub("B",""):gsub("%s"," "):gsub("K","k")
      local size_fmt   	= size:pgsub("\u{00A0}bytes",""):pgsub("\u{00A0}","\u{2007}\u{2007}"):pgsub("B",""):pgsub("K","k") -- '3 KB'→'3 KB' -- 00A0→ 200A   2007
      local size_byte  	= string.byte(size_fmt)
      entity = name .. "\t(" .. size_fmt .. ")"
      -- martax module already provides a helper function for rendering size so we don’t need to write it by ourselves Prints "1 KB" martax.formatSize(1024)
    end

    text = text .. entity .. "\n"
  end

  martax.alert("Files: \n\n" .. text)
end
})

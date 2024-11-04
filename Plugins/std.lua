local std = {} -- stdlib

-- escape magic chars ^$()%.[]*+-? https://www.lua.org/manual/5.1/manual.html#5.4.1
string.escmagic  	= function(self, str)	return self:gsub("([^%w])", "%%%1")                  	end
string.startswith	= function(self, str)	return self:find('^' .. str:escmagic()       ) ~= nil	end
string.endswith  	= function(self, str)	return self:find(       str:escmagic() .. '$') ~= nil	end
string.isin      	= function(self, str)	return str:find(       self:escmagic()       ) ~= nil	end
string.trim      	= function(self     )	return self:match("^%s*(.-)%s*$")                    	end
string.split     	= function(self, sep)
  if sep == nil then sep = ',' end
  local ret = {}
  for str in string.gmatch(self, "([^"..sep.."]+)") do
    ret[#ret + 1] = str end
  return ret
end
string.splitflex	= function(self, sep)
  local seps = {',',';'}
  if sep == nil then
    for k,v in pairs(seps) do
      if self:find(v) then sep = v end end
    if sep == nil then sep = ' ' end -- finally, use space
  end
  local ret = {}  -- replace tabs with spaces↓
  for str in string.gmatch(self:gsub('\t',' '), "([^"..sep.."]+)") do
    ret[#ret + 1] = str end
  return ret
end

function std.getScriptFullPath()
  local source = debug.getinfo(2,"S").source
  if source:sub(1,1) == "@" then
    local path_arg = source:sub(2)
    -- ↓ os-dependent resolution of relative paths, but we pass absolute paths
    -- local fullpath = io.popen("realpath '"..path_arg.."'",'r'):read('a')
    -- fullpath = fullpath:gsub('[\n\r]*$','')
    return path_arg
  else error("Caller was not defined in a file", 2) end
end
function std.dir_filename(fullpath)
  if type(fullpath) ~= "string" then return nil end
  local dirname, filename = fullpath:match('^(.*[/\\])([^/\\]-)$')
  dirname 	= dirname  or ''
  filename	= filename or fullpath
  return dirname, filename
end
function std.dir(        fullpath)
  if type(fullpath) ~= "string" then return nil end
  local dirname, filename = fullpath:match('^(.*[/\\])([^/\\]-)$')
  return dirname  or ''
end
function std.filename(   fullpath)
  if type(fullpath) ~= "string" then return nil end
  local dirname, filename = fullpath:match('^(.*[/\\])([^/\\]-)$')
  return filename or fullpath
end
function std.delua(filename) -- strip script's extension
  if type(filename) ~= "string" then return nil end
  return filename:gsub("(.*)(.lua)"    ,"%1") -- (file)(.lua)  → (file)
end

function std.basename(path) -- get filename from path
  if type(path) ~= "string" then return nil end
  return path:gsub("(.*/)(.*)"    ,"%2") -- replace (path/)(file)          with (file)
end

-- Print contents of `tbl`, with indentation `indent` gist.github.com/ripter/4270799
function std.tprint(tbl, indent)
  if not indent then indent = 0 end
  for k, v in pairs(tbl) do
    key = string.rep("  ",indent)..k..": "
    if     type(v) == "table"   then	print(key             ); std.tprint(v,indent+1)
    elseif type(v) == 'boolean' then	print(key..tostring(v))
    else                            	print(key..         v ) end
  end
end
-- Print contents of `tbl`, with indentation `indent`, to string
function std.t2str(tbl, indent, out)
  if not indent then indent = 0  end
  if not out    then out    = '' end
  for k, v in pairs(tbl) do
    key = string.rep("  ",indent)..k..": "
    if     type(v) == "table"   then	out = out..key             ..'\n'; out=std.t2str(v,indent+1,out)
    elseif type(v) == "boolean" then	out = out..key..tostring(v)..'\n'
    else                            	out = out..key..         v ..'\n' end
  end
  return out
end

function std.tValOrSelf(tab, key)   -- return value if key matches otherwise return self
  for k, val in  pairs(tab) do
    if k == key     then return tab[key]  end
  end
                         return     key
end
function std.tHasKey(tab, key)   -- check whether a table has key
  for k, val in  pairs(tab) do
    if k == key     then return true  end
  end
                         return false
end
function std.tHasVal(tab, value) -- check whether a table has value
  for i, val in ipairs(tab) do
    if val == value then return true  end
  end
                         return false
end

function std.tlen(T)
  local count = 0
  for _ in pairs(T) do count = count + 1 end
  return count
end

-- Math
-- Round @ https://stackoverflow.com/a/58411671
function std.round(num)
  -- if Lua uses double precision IEC-559 (aka IEEE-754) floats (most do)
  -- if -2⁵¹ < num < 2⁵¹
  -- rounding using your FPU's current rounding mode, which is usually round to nearest, ties to even
  return num + (2^52 + 2^51) - (2^52 + 2^51)
end
function std.roundp(num) -- less efficient, performs the same FPU rounding but works for all numbers
  -- doesn't seem to differ, still fails at 0.5=0
  local ofs = 2^52
  if math.abs(num) > ofs then
    return num
  end
  return num < 0 and num - ofs + ofs or num + ofs - ofs
end

local ESC	= "\x1b"	-- Escape, starts all the escape sequences
std.EscapeCodes = { -- https://iterm2.com/documentation-escape-codes.html equivalents
  ESC	= "\x1b"     	, -- Escape, starts all the escape sequences
  BEL	= "\x07"     	, -- Bell
  ST 	= ESC .. "\\"	, -- 0x9C String Terminator: terminates S in other controls
  OSC	= ESC .. "]" 	, -- \x5d Operating System Command
  CSI	= ESC .. "[" 	, --      Control Sequence Introducer
  DCS	= ESC .. "P" 	, -- 0x90 Device Control String (terminated by ST)
}

return {
  std = std,
}

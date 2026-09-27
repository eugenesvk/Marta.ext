local plugID = "es¦file"
marta.plugin({id=plugID,name="File information",apiVersion="2.2"})
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

-- helper functions
function dec_to_oct(n) -- converts decimal to octal
  local o, p = 0, 1
  while n > 0 do
    o = o + (n & 7) * p
    n =     n >> 3
    p = p * 10
  end
  return o
end
local function dide(n, i) -- gets Ith digit of a decimal number (from the right, missing digits = 0)
  n = math.abs(n)
  local p = 10 ^ (i - 1)
  -- if n < p then return nil end -- don't break on missing digits
  return math.tointeger((n // p) % 10)
end
local function t_copy1(obj) -- simlpe copy, ignores metatables and recursive tables
  if type(obj) ~= 'table' then return obj end
  local res = {}
  for k, v in pairs(obj) do res[t_copy1(k)] = t_copy1(v) end
  return res
end
local function get_uid() -- gets logged in user id, NOT uid of the current process (which can be root 0)
  local u_gui_f = marta.localFileSystem:get('/dev/console')
  if u_gui_f:exists() then
    local err,FI = u_gui_f:readInfo("stat")
    if FI then return FI.ownerId,FI.groupId end
  end
end

names_U = {}
names_G = {}
local function get_uname(idU)
  local nmU,nmG = names_U[idU],names_G[idU]
  local handle
  if nmU == nil then
    handle = io.popen("id -nu "..tostring(idU)); nmU = handle:read("*a"):gsub("%s+",""); handle:close()
    names_U[idU] = nmU;   end
  if nmG == nil then
    handle = io.popen("id -ng "..tostring(idU)); nmG = handle:read("*a"):gsub("%s+",""); handle:close()
    names_G[idU] = nmG;   end
  return nmU,nmG
end

local nmU_sub = {pete_longinamignol='👶'} -- replace some user names with shorter names or symbols
local nmG_sub = {staff='𝕊'} -- replace some user names with shorter names or symbols

marta.action({id="ℹnf",name="Show file information",
  isApplicable = function(ctxA) return ctxA.activePane.model.hasActiveFiles end,
  apply        = function(ctxA) info({ctxA=ctxA})  ; end})

local _d = 0

function info(arg)
  local ctxA 	= arg.ctxA	-- holds refs to PaneContext instances for active+inactive panes
  local ctxW 	= ctxA.window
  local ctxG 	= marta.globalContext
  local actG 	= ctxG.actions
  local arg_u	= ctxA.args	-- arguments passed from users when invoking the action

  local ctxPA   	= ctxA.activePane       	-- ctxA holds refs to PaneContext instances for active+inactive panes
  local list_m  	= ctxPA.model           	-- Active pane list model
  local files   	= list_m.activeFiles    	-- array of File objects which are bare pointers to files, no attributes (not all fs store them, ZIP doesn't store macOS extended attributes)
  local filesInf	= list_m.activeFileInfos	-- array of FileInfo with all the attributes, gathered on folder load, so cached (ZIP fs doesn’t store macOS extended attributes, so isApplication will always return false)
  local viewP   	= ctxPA.view

  function pss(msg) viewP:showNotification(msg,plugID,"short") end -- short-term "print" → statusbar
  function psl(msg) viewP:showNotification(msg,plugID,"long" ) end -- long -term "print" → statusbar
  function dbg(l,msg) if (_d>=l) then pss(msg) end end -- short-term "print" to the statusbar if dbg≥level
  function _d0(  msg) if (_d>=0) then pss(msg) end end
  function _d1(  msg) if (_d>=1) then pss(msg) end end
  function _d2(  msg) if (_d>=2) then pss(msg) end end
  function _d3(  msg) if (_d>=3) then pss(msg) end end

  if #filesInf == 0 then _d1("es¦file: no selection") return end -- skip an empty dir

  local P, V, Cg, Ct, Cc, S, R, C, Cf, Cb, Cs = -- save typing function names with "lpeg" in front of them:
    lpeg.P, lpeg.V, lpeg.Cg, lpeg.Ct, lpeg.Cc, lpeg.S, lpeg.R, lpeg.C, lpeg.Cf, lpeg.Cb, lpeg.Cs
  -- character classes
  lpeg.locale (lpeg)  -- get digit, alpha, etc.
  local alpha, cntrl, digit, graph, lower, punct, space, upper, alnum, xdigit =
    lpeg.alpha, lpeg.cntrl, lpeg.digit, lpeg.graph, lpeg.lower, lpeg.punct,
    lpeg.space, lpeg.upper, lpeg.alnum, lpeg.xdigit
  -- Now, the “P” function makes a pattern according to what you supply it. The simple case is a string literal, so that P“foo” matches “foo”.

  -- -- local p = lpeg.R"az"^1 * -1 -- matches a word followed by end-of-string
  -- local p = R"az"^1 * -1 -- matches a word followed by end-of-string
  -- local res1 = p:match   (   "hello") --> 6
  -- local res2 = lpeg.match(p, "hello") --> 6
  -- local res3 = p:match   ( "1 hello") --> nil
  -- -- martax.alert(uuid() .. "\n" .. tostring(res1) .. tostring(res2) .. tostring(res3))

  local text = ""
  local hd_perm = "👤👨‍👧‍👦∀" -- user, group, others   👥
  --   r	w	rw	e    single char permissions. For dirs exe bit means can search inside
    -- ⇧	⇩	⇳ 	  (no  exe)
    -- ⬆	⬇	⬍ 	•   (yes exe)
    -- 4	2	6 	1 (no  exe)
    -- 5	3	7 	  (yes exe)
  local oct_to_sym = {[0]=" ",[1]="•"
    ,                 [2]="⇩",[3]="⬇"
    ,                 [4]="⇧",[5]="⬆"
    ,                 [6]="⇳",[7]="⬍"}
  local oct_to_sym_dir = t_copy1(oct_to_sym)
  oct_to_sym_dir[1]="⌕" --🔍🔎⌕

  local b_sticky = 0x1
  local idU,idG = get_uid()
  local nmU,nmG = get_uname(idU)
  -- martax.alert(tostring(idU)..'='..nmU,tostring(idG)..'='..nmG)

  for i, tgtFI in ipairs(filesInf) do -- Iterate thru active=(selected¦cursor) files
    local name = tgtFI.name
    local entity

    -- todo: detect alias to folder, see my es fd open plugin
    local perm = ""
    local pos = ""
    --check if info isn't contained, then do readinfo to get it:    local _, info = target:readInfo("node"). FileInfoField arg of values
      -- owner: Owner information presence → ownerId, groupId
      -- node : Node  information presence → deviceId, inodeId, hardLinkCount
      -- mode : POSIX mode presence (value mode)
      -- stat ≝ node, mode, owner, size, dateCreated, dateModified
    -- stat -f %Sf @(f) → detects 'uchg' flag ("locked" in Finder), todo: does this exist in Marta
    -- todo: detect sticky bit: sticky text and append-only directories
    local is_dir = tgtFI.isFolder -- TODO: update for aliases??? or use separate column for alias targets

    -- groupId, ownerId - show if not expected (user/staff)
    -- todo: convert to names and don't show if expected user/staff
    -- if not tgtFI:contains("owner") then
    --   -- local file = tgtFI.file
    --   martax.alert("file.name.rawValue")
    --   -- local err, FI = file:readInfo("stat")
    --   -- if err then             martax.alert("NO owner, NO stat") end
    --   -- if FI  then tgtFI = FI; martax.alert("NO owner, but got stat") end
    -- end
    -- todo: get UID of logged in user and ignore that one instead of blindly 501
      -- todo would need FFI to get from The OS, lua can't do that, use Rust
    -- Get Owner/Group info
    local owner = ''
    local nU,nG, got_UG = nil,nil, false
    local  id=tgtFI.ownerId
    if     id==  0 then owner='#'
    elseif id==idU then owner='' -- logged in user, expected, show blank
    elseif id==201 then owner='𝕘' -- 👶guest
    else
      if not got_UG and nU==nil then
        nU,nG = get_uname(id); got_UG=true end
      if nU~=nil then if      nmU_sub[nU]~=nil
        then            owner=nmU_sub[nU] -- martax.alert('sub user name'..nU..'with '..nmU_sub[nU])
        else            owner=nU end
      else              owner='👤' end
    end
    local group = '' -- list of user names 'dscl . -list /Users PrimaryGroupID'
    id=tgtFI.groupId
    if     id==  0 then group='⚙'
    elseif id==  1 then group='☠'  -- daemon 👿👹☠
    elseif id==idG then group=''  -- 👥 𝕤𝕊 staff, expected, don't show  (20)
    elseif id==201 then group='𝔾' -- 👶guest
    elseif id== 80 then group='⛏' -- admin
    else
      if not got_UG and nG==nil then
        nU,nG = get_uname(id); got_UG=true end
      if nG~=nil then if      nmU_sub[nG]~=nil
        then            group=nmU_sub[nG]
        else            group=nG end
      else              group='👨‍👧‍👦' end
    end
    pos = owner..group..pos
    -- "👤"..tostring(tgtFI.ownerId).."👨‍👧‍👦"..tostring(tgtFI.groupId)..pos
    -- #root √🔧👷🫚🫜 ⛨🛡  👤 👥  👨‍👧‍👦   ⚙wheel ⛏admin
    -- isPackage
    if tgtFI.isSymbolicLink    then pos=pos.."🔗 " else -- order matters synce symlink are type of isAlias
    if tgtFI.isAlias           then pos=pos.."⤻ "  else
    if not is_dir              and
       tgtFI.hardLinkCount > 1 then pos=pos.."⤑"..tostring(tgtFI.hardLinkCount)
      else                          pos=pos.."  " end   end end
    if _d>=3 then psl("mode = 0d"..tostring(tgtFI.mode).." 0o"..tostring(dec_to_oct(tgtFI.mode)).." hard⤑"..tostring(tgtFI.hardLinkCount).." sym🔗"..tostring(tgtFI.isSymbolicLink).." alias⤻"..tostring(tgtFI.isAlias).." 👤"..nmU..idU.." "..tostring(tgtFI.ownerId).."👨‍👧‍👦"..nmG..idG.." "..tostring(tgtFI.groupId)) end

    -- Get permissions
    local m_oct = dec_to_oct(tgtFI.mode)
    local d,a,g,o,s
    local lookup = oct_to_sym
    d = dide(m_oct,4)
    if is_dir then lookup = oct_to_sym_dir
      if ((d & b_sticky) == b_sticky) then s="🗀" else s="🗁" end   else
      if ((d & b_sticky) == b_sticky) then s="🝉" else s=" "  end -- todo: ignore files? has no effect on files
    end
    d = dide(m_oct,1); a = lookup[d] --all
    d = dide(m_oct,2); g = lookup[d] --group
    d = dide(m_oct,3); o = lookup[d] --owner
    -- d = dide(m_oct,4); s = " "; if ((d & b_sticky) == b_sticky) then s="🝉" end -- ignore for files (useless)
    if _d>=4 then pos="≝"..tostring(m_oct % 1000)..pos else end
    pos = o..g..a..""..s..""..pos
    entity = pos.."¦"..name
    if is_dir then
    else
      -- isExecutable
      local size       	= martax.formatSize(tgtFI.size)
      -- local size_fmt	= size:gsub("bytes",""):gsub("B",""):gsub("%s"," "):gsub("173","")
      -- local size_fmt	= size:gsub("bytes",""):gsub("B",""):gsub("%s"," "):gsub("K","k")
      local size_fmt   	= size:pgsub("\u{00A0}bytes",""):pgsub("\u{00A0}","\u{2007}\u{2007}"):pgsub("B",""):pgsub("K","k") -- '3 KB'→'3 KB' -- 00A0→ 200A   2007
      local size_byte  	= string.byte(size_fmt)
      entity = entity.."\t(" .. size_fmt .. ")"
      -- martax module already provides a helper function for rendering size so we don’t need to write it by ourselves Prints "1 KB" martax.formatSize(1024)
    end

    text = text .. entity .. "\n"
  end

  -- martax.alert("foo", "bar", {"Ok", "Cancel"}, "informational")
  -- strange bug
    -- failed: Unexpected argument count: [1, 2, 3, 4] expected, got 1, 2, 3, 4
  martax.alert(text)
end

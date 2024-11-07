-- https://stackoverflow.com/a/39274330
local function packed_args_append(packed, nb_insert, ...)
  nb_insert = nb_insert > 0 and nb_insert or select('#', ...)
  for i = 1, nb_insert do
    packed[packed.n + 1] = select(i, ...)
    packed.n = packed.n + 1
  end
end

-- placeholder_format string format:
  -- . placeholder for future call
  -- A given argument
  -- ~ the rest of the varargs
-- example: function myfunc(a, b, c, d) print(a, b, c, d) end
  -- local newfunc = myfunc:bind("...A", 42)
  -- newfunc(1, 2, 3)
  -- => prints: "1 2 3 42"
-- If you don't know how many argument there is after the last known argument, put '~' in the placeholder_format string
-- last varargs with nil example:
  -- local debug_print = print:bind("A.A~", "DEBUG:", "OTHER")
  -- debug_print(1, 2, nil, 4)
  -- => prints: "DEBUG: 1 OTHER 2 nil 4"
-- implementation details for debug_print :
  -- -> placeholders == {n = 2, "DEBUG:", "OTHER"}
  -- -> ... == 1, 2, nil, 4
  -- -> args == {n = 6, "DEBUG:", 1, "OTHER", 2, nil, 4}

-- replace table.unpack as it doesn't handle nil values at the end of varargs.., ex:
  -- table.pack(1, 2, nil) is the unpacked the same as:
  -- table.pack(1, 2)
local function unpack_w_end_nil(packed)
  local nb_args = packed.n
  local function unpack_n(n)
    if n >= nb_args then
      return packed[n]                  end
    return   packed[n], unpack_n(n + 1) end
  return                unpack_n(    1)
end

local function multi_unpack(...)
  local nb_packed = select("#", ...)

  if nb_packed == 0 then return nil                              end
  if nb_packed == 1 then return unpack_w_end_nil(select(1, ...)) end

  local all_packed = {n = 0}
  for i = 1, nb_packed do
    local current_packed = select(i, ...)
    for arg_no = 1, current_packed.n do
      all_packed.n = all_packed.n + 1
      all_packed[all_packed.n] = current_packed[arg_no]
    end
  end

  return unpack_w_end_nil(all_packed)
end

local function function_binder(self, placeholder_format, ...)
  local placeholders = table.pack(...)

  return function(...)
    local args            = {n = 0}
    local arg_idx         = 1
    local placeholder_idx = 1

    for c in placeholder_format:gmatch "." do
      if     c == 'A' then packed_args_append(args,  1, placeholders[placeholder_idx])
        placeholder_idx = placeholder_idx + 1
      elseif c == '.' then packed_args_append(args,  1, select(arg_idx, ...))
        arg_idx = arg_idx + 1
      elseif c == '~' then packed_args_append(args, -1, select(arg_idx, ...)); break; end
    end
    return self(unpack_w_end_nil(args))
  end
end

local function function_partial(self, ...)
  local placeholders = table.pack(...)

  return function(...)
    local args = table.pack(...)
    return self(multi_unpack(placeholders, args))
  end
end

local function_mt = {
  __index = {
    bind    = function_binder,
    partial = function_partial,
  },
}

debug.setmetatable(function() end, function_mt)

--[[ try
local function add(a, b) return a + b end
local add3 = add:bind("A.", 3)
print(add3(3))
print(add3(0))

print("================")
print("Bind")
local debug_print = print:bind("A.A~", "DEBUG:", "OTHER")
debug_print(1, 2, nil, 4)

debug_print()

print("================")
print("Partial: single arg")
local partial_debug = print:partial("DEBUG:")

partial_debug("my value")
partial_debug("some", "other", "values")

print("================")
print("Partial: multi args")
partial_debug = print:partial("[LOG]", "DEBUG:")

partial_debug("my value")
partial_debug("some", "other", "values")

--print("================")
--print("Remove args of function")
--local remove_args = partial_debug:bind("")
--remove_args(1, 2, 3, 4)
--]]

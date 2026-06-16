modes = {}

function define_mode(name, config)
    modes[name] = config
end

function bind(f, ...)
  local args = {...}
  return function() f(table.unpack(args)) end
end

function inherit(extra, base)
  return setmetatable(extra, {__index = base})
end

local _rawset = rawset
local _rawget = rawget
local _error = error
local _G = _G

local declared = {}

function strict_mode()
  -- pre-declare all existing globals so they can still be used
  for k in pairs(_G) do
    declared[k] = true
  end

  setmetatable(_G, {
    __newindex = function(t, k, v)
      if not declared[k] then
        _error("attempt to assign to undeclared global '" .. k .. "'", 2)
      end
      _rawset(t, k, v)
    end,
    __index = function(t, k)
      if not declared[k] then
        _error("attempt to read undeclared global '" .. k .. "'", 2)
      end
      return _rawget(t, k)
    end
  })
end

function global(name)
  declared[name] = true
end
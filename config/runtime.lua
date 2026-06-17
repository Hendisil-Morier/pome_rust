-- config/runtime.lua

-- Global mode registry
modes = {}

function define_mode(name, config)
    modes[name] = config
end

-- Helpers
function bind(f, ...)
    local args = { ... }
    return function() f(table.unpack(args)) end
end

function inherit(extra, base)
    return setmetatable(extra, { __index = base })
end

-- Import the key builder
key = require("key_builder")
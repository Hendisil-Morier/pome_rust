-- config/key_builder.lua

local modifier_set   = { alt = true, ctrl = true, shift = true }
local modifier_order = { alt = 1, ctrl = 2, shift = 3 }

local function finalize(mods, base)
    table.sort(mods, function(a, b)
        return (modifier_order[a] or 999) < (modifier_order[b] or 999)
    end)
    local parts = {}
    for _, m in ipairs(mods) do
        parts[#parts + 1] = m
    end
    parts[#parts + 1] = base
    return table.concat(parts, "+")
end

local function make_builder(modifiers)
    local builder = {}
    local mt = {
        __index = function(_, k)
            if modifier_set[k] then
                local new_mods = { table.unpack(modifiers) }
                new_mods[#new_mods + 1] = k
                return make_builder(new_mods)
            else
                return finalize(modifiers, k)
            end
        end,
        __call = function(_, base)
            return finalize(modifiers, base)
        end,
    }
    setmetatable(builder, mt)
    return builder
end

local key = setmetatable({
    left      = "arrow_left",
    right     = "arrow_right",
    up        = "arrow_up",
    down      = "arrow_down",
    enter     = "enter",
    backspace = "backspace",
    delete    = "delete",
    esc       = "esc",
}, {
    __index = function(_, k)
        if modifier_set[k] then
            return make_builder({ k })
        end
        return nil
    end,
    __call = function(_, base)
        return base
    end,
})

-- Plain alphanumeric characters
for i = string.byte("a"), string.byte("z") do
    key[string.char(i)] = string.char(i)
end
for i = string.byte("A"), string.byte("Z") do
    key[string.char(i)] = string.char(i)
end
for i = string.byte("0"), string.byte("9") do
    key[string.char(i)] = string.char(i)
end

return key
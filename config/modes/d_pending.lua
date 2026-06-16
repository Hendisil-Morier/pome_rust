-- modes/d_pending.lua
local std = require("stdlib")
local base = require("modes.base_keymap")

local function make_d_pending_config()
  local keymap = inherit(
    {
      [key_press.d] = std.delete_line,   -- dd → delete whole line
    },
    base
  )
  return {
    minor  = true,
    keymap = keymap,
  }
end

return make_d_pending_config
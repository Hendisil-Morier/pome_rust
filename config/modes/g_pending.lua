-- modes/g_pending.lua
local std = require("stdlib")
local base = require("modes.base_keymap")

local function make_g_pending_config()
  local keymap = inherit(
    {
      [key_press.g] = bind(std.goto_firstline),   -- gg → top of file
    },
    base
  )
  return {
    minor  = true,     -- tells Pome this is a minor (pending) mode
    keymap = keymap,
  }
end

return make_g_pending_config
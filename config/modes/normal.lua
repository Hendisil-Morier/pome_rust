-- modes/normal.lua
local std = require("stdlib")
local base = require("modes.base_keymap")

local function make_normal_config()
  local keymap = inherit(
    {
      [key.h] = std.cursor_left,
      [key.l] = std.cursor_right,
      [key.k] = std.cursor_up,
      [key.j] = std.cursor_down,

      [key.d] = bind(std.enter_minor_mode, "d_pending"),
      [key.g] = bind(std.enter_minor_mode, "g_pending"),
      [key.i] = bind(std.safe_set_mode, "insert"),
      [key.v] = bind(std.safe_set_mode, "visual"),
      [key.w] = std.word_forward,
    },
    base
  )
  return {
    keymap = keymap,
  }
end

return make_normal_config
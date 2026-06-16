-- modes/normal.lua
local std = require("stdlib")
local base = require("modes.base_keymap")

local function make_normal_config()
  local keymap = inherit(
    {
      [key_press.h] = std.cursor_left,
      [key_press.l] = std.cursor_right,
      [key_press.k] = std.cursor_up,
      [key_press.j] = std.cursor_down,

      [key_press.d] = bind(std.enter_minor_mode, "d_pending"),
      [key_press.g] = bind(std.enter_minor_mode, "g_pending"),
      [key_press.i] = bind(std.safe_set_mode, "insert"),
      [key_press.v] = bind(std.safe_set_mode, "visual"),
      [key_press.w] = std.word_forward,
    },
    base
  )
  return {
    keymap = keymap,
  }
end

return make_normal_config
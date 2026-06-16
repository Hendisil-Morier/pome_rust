-- modes/insert.lua
local std = require("stdlib")
local base = require("modes.base_keymap")

local function make_insert_config()
  local keymap = inherit(
    {
      [key_press.backspace]  = std.delete_before_cursor,
      [key_press.backspace2] = std.delete_before_cursor,   -- some terminals
      [key_press.delete]     = std.delete_after_cursor,
      [key_press.enter]      = std.insert_newline,
    },
    base
  )

  return {
    default = function(ch) std.insert_char(ch) end,
    keymap  = keymap,
  }
end

return make_insert_config
-- modes/insert.lua
local std = require("stdlib")
local base = require("modes.base_keymap")

local function make_insert_config()
  local keymap = inherit(
    {
      [key.backspace]  = std.delete_before_cursor,
      [key.delete]     = std.delete_after_cursor,
      [key.enter]      = std.insert_newline,
    },
    base
  )

  return {
    default = function(ch) std.insert_char(ch:byte()) end,
    keymap  = keymap,
  }
end

return make_insert_config
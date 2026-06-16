-- modes/visual.lua
local std = require("stdlib")
local base = require("modes.base_keymap")

local function make_visual_config()
  local keymap = inherit(
    {
      [key_press.h] = std.cursor_left,
      [key_press.l] = std.cursor_right,
      [key_press.k] = std.cursor_up,
      [key_press.j] = std.cursor_down,

      [key_press.d] = function()
        pome.delete_selected()
        pome.clear_anchor()
        std.safe_set_mode("normal")
      end,

      [key_press.v] = bind(std.safe_set_mode, "normal"),
    },
    base
  )

  return {
    on_enter = pome.set_anchor,
    on_exit  = pome.clear_anchor,
    keymap   = keymap,
  }
end

return make_visual_config
-- modes/visual.lua
local std = require("stdlib")
local base = require("modes.base_keymap")

local function make_visual_config()
  local keymap = inherit(
    {
      [key.h] = std.cursor_left,
      [key.l] = std.cursor_right,
      [key.k] = std.cursor_up,
      [key.j] = std.cursor_down,

      [key.d] = function()
        pome.delete_selected()
        pome.clear_anchor()
        std.safe_set_mode("normal")
      end,

      [key.v] = bind(std.safe_set_mode, "normal"),
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
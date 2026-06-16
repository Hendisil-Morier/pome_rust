-- modes/base_keymap.lua
local std = require("stdlib")
local safe_set_mode = std.safe_set_mode

return {
  [key_press.arrow_left]  = std.cursor_left,
  [key_press.arrow_right] = std.cursor_right,
  [key_press.arrow_up]    = std.cursor_up,
  [key_press.arrow_down]  = std.cursor_down,

  [key_press.esc]         = bind(safe_set_mode, "normal"),
  [key_press.ctrl_r]      = bind(pome.load_config, "config.init.lua"),
  [key_press.ctrl_q]      = pome.quit_editor,
  [key_press.ctrl_s]      = pome.save_file,
}
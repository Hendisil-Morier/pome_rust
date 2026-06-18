local std = require("stdlib")
local safe_set_mode = std.safe_set_mode

return {
  [key.left]       = std.cursor_left,
  [key.right]      = std.cursor_right,
  [key.up]         = std.cursor_up,
  [key.down]       = std.cursor_down,

  [key.esc]        = bind(safe_set_mode, "normal"),
  [key.ctrl.r]  = bind(pome.load_config, "config/init.lua"),
  [key.ctrl.q]  = pome.quit_editor,
  [key.ctrl.s]  = pome.save_file,
}
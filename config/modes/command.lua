-- modes/command.lua
local base = require("modes.base_keymap")

local cmd_text = ""

local function make_command_config()
  local keymap = inherit(
    {
      [key.backspace] = function()
        if #cmd_text > 0 then
          cmd_text = cmd_text:sub(1, -2)
        end
      end,
      [key.enter] = function()
        if cmd_text == "w" then
          pome.save_file()
        elseif cmd_text == "q" then
          pome.quit_editor()
        elseif cmd_text == "wq" then
          pome.save_file()
          pome.quit_editor()
        end
        pome.set_mode("normal")
      end,
      [key.esc] = function()
        pome.set_mode("normal")
      end,
    },
    base
  )

  return {
    default = function(ch)
      cmd_text = cmd_text .. ch
    end,
    keymap = keymap,
    on_enter = function() 
      cmd_text = ""
      pome.set_cursor_shape("bar_blink") 
    end,
    -- We expose this so engine.lua can draw it!
    get_text = function()
      return cmd_text
    end,
  }
end

return make_command_config

-- stdlib/editing.lua
-- Basic editing operations that combine cursor movement with Pome API calls
local motion = require("stdlib.motion")

-- We'll use motion.cursor_line_start, but we can also use other helpers if needed
-- (like motion.move_cursor_to, motion.pos_next, etc.)

local M = {}

-- Delete the entire current line
function M.delete_line()
  -- Move to the beginning of the line first (this also updates prefer_x)
  motion.cursor_line_start()

  local _, y = pome.get_cursor_pos()

  -- Anchor at the start of the next line (selects the whole line)
  pome.set_anchor(0, y + 1)
  pome.delete_selected()
  pome.clear_anchor()
end

-- Insert a newline at the cursor (splits the line)
function M.insert_newline()
  pome.insert_char('\n')
end

-- Simple character deletion (forward and backward)
-- You can either wrap the Pome API directly or add small safety checks.
function M.delete_before_cursor()
  pome.delete_before()
end

function M.delete_after_cursor()
  pome.delete_after()
end

-- Insert a character at the cursor (used by insert mode's default handler)
function M.insert_char(ch)
  pome.insert_char(ch)
end

-- Example of a more complex edit: delete to end of line
function M.delete_to_line_end()
  local x, y = pome.get_cursor_pos()
  local line_end = pome.get_line_end(y)
  if not line_end then return end
  if x == line_end then
    -- Nothing to delete
    return
  end
  pome.set_anchor(x, y)
  pome.move_cursor_to(line_end, y)
  pome.delete_selected()
  pome.clear_anchor()
end

-- Example: delete word forward (could combine motion and deletion)
-- (Not fully implemented, just a sketch)
-- function M.delete_word_forward()
--   -- set anchor, call word_forward to extend selection, then delete
-- end

return M
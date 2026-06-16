-- stdlib/init.lua
-- Aggregates all stdlib modules into one table for easy import

local charset = require("stdlib.charset")
local motion  = require("stdlib.motion")
local editing = require("stdlib.editing")
local mode    = require("stdlib.mode")

return {
  -- charset
  make_charset   = charset.make_charset,
  merge_sets     = charset.merge_sets,
  whitespace     = charset.whitespace,
  identifier     = charset.identifier,
  non_word       = charset.non_word,

  -- motion (cursor + word movements)
  move_cursor_to   = motion.move_cursor_to,
  cursor_up        = motion.cursor_up,
  cursor_down      = motion.cursor_down,
  cursor_left      = motion.cursor_left,
  cursor_right     = motion.cursor_right,
  cursor_line_start= motion.cursor_line_start,
  cursor_line_end  = motion.cursor_line_end,
  word_forward     = motion.word_forward,
  word_backward    = motion.word_backward,
  goto_firstline   = motion.goto_firstline,
  goto_lastline    = motion.goto_lastline,
  pos_prev         = motion.pos_prev,
  pos_next         = motion.pos_next,

  -- mode management
  safe_set_mode    = mode.safe_set_mode,
  enter_minor_mode = mode.enter_minor_mode,
  
  --editing
  delete_line          = editing.delete_line,
  insert_newline       = editing.insert_newline,
  delete_before_cursor = editing.delete_before_cursor,
  delete_after_cursor  = editing.delete_after_cursor,
  insert_char          = editing.insert_char,
  delete_to_line_end   = editing.delete_to_line_end,  -- if you want it  
}
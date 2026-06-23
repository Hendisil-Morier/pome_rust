-- stdlib/motion.lua
-- Cursor movement, word motions, and position helpers
local charset = require("stdlib.charset")
local state = require("stdlib.state")

-- Pull in character sets we need
local identifier = charset.identifier
local whitespace = charset.whitespace
local non_word   = charset.non_word

-- =====================================================
-- Internal helpers (not exported directly)
-- =====================================================

-- Move one character backward; returns nil if at start of file
local function pos_prev(x, y)
  if not x or not y then return end

  if x > 0 then
    return x - 1, y
  end
  if y == 0 then
    return nil
  end

  local prev_x = pome.get_line_end(y - 1)
  return prev_x, y - 1
end

-- Move one character forward; returns nil if at end of file
local function pos_next(x, y)
  local line_end = pome.get_line_end(y)
  if x < line_end then
    return x + 1, y
  end

  local total_lines = pome.get_max_line_index() + 1
  if y + 1 < total_lines then
    return 0, y + 1
  end

  return nil
end

-- Safe cursor placement with clamping
local function move_cursor_to(x, y)
  if not x or not y then return end

  local line_end = pome.get_line_end(y)
  if not line_end then return end
  if x > line_end then
    x = line_end
  end

  pome.move_cursor_to(x, y)
end

-- =====================================================
-- Public movement functions
-- =====================================================

local M = {}

function M.cursor_up()
  local _, y = pome.get_cursor_pos()
  move_cursor_to(state.prefer_x, y - 1)
end

function M.cursor_down()
  local _, y = pome.get_cursor_pos()
  move_cursor_to(state.prefer_x, y + 1)
end

function M.cursor_left()
  pome.move_cursor("left", 1)
  state.prefer_x = pome.get_cursor_pos()
end

function M.cursor_right()
  pome.move_cursor("right", 1)
  state.prefer_x = pome.get_cursor_pos()
end

function M.cursor_line_start()
  local _, y = pome.get_cursor_pos()
  pome.move_cursor_to(0, y)
  state.prefer_x = 0
end

function M.cursor_line_end()
  local _, y = pome.get_cursor_pos()
  local line_end = pome.get_line_end(y)
  move_cursor_to(line_end, y)
  state.prefer_x = line_end
end

function M.goto_firstline()
  move_cursor_to(state.prefer_x, 0)
end

function M.goto_lastline()
  move_cursor_to(state.prefer_x, pome.get_max_line_index())
end

-- =====================================================
-- Word motions (character-class aware)
-- =====================================================

function M.word_forward()
  local x, y = pome.get_cursor_pos()
  local byte = pome.char_at(x, y)
  if byte == nil then return end

  local nx, ny

  if identifier[byte] then
    nx, ny = pome.forward_match_notset(identifier, x, y)
  elseif whitespace[byte] then
    nx, ny = pome.forward_match_notset(whitespace, x, y)
    if nx == nil then return end
    move_cursor_to(nx, ny)
    state.prefer_x = nx
    return
  else
    nx, ny = pome.forward_match_set(non_word, x, y)
  end

  if nx == nil then return end

  nx, ny = pome.forward_match_notset(whitespace, nx, ny)
  if nx == nil then return end

  move_cursor_to(nx, ny)
  state.prefer_x = nx
end

function M.word_backward()
  local x, y = pome.get_cursor_pos()

  -- step one character backward first
  local nx, ny = pos_prev(x, y)
  if nx == nil then return end

  -- skip backward over any whitespace
  nx, ny = pome.backward_match_notset(whitespace, nx, ny)
  if nx == nil then return end

  local byte = pome.char_at(nx, ny)
  if byte == nil then return end

  local bx, by
  if identifier[byte] then
    bx, by = pome.backward_match_notset(identifier, nx, ny)
  else
    bx, by = pome.backward_match_set(non_word, nx, ny)
  end

  if bx ~= nil then
    nx, ny = pos_next(bx, by)
    if nx == nil then return end
  else
    nx, ny = 0, 0
  end

  move_cursor_to(nx, ny)
  state.prefer_x = nx
end

-- Expose internal helpers in case other modules need them
M.pos_prev = pos_prev
M.pos_next = pos_next
M.move_cursor_to = move_cursor_to   -- already used internally, but keep public

return M
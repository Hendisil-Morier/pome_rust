function safe_set_mode(name)
  if (name ~= pome.get_mode())
  then pome.set_mode(name)
  end
end

function make_charset(str)
  local set = {}
  for i = 1, #str do
    set[string.byte(str, i)] = true
  end
  return set
end

function merge_sets(...)
  local result = {}
  for _, set in ipairs({...}) do
    for k, v in pairs(set) do
      result[k] = v
    end
  end
  return result
end

function enter_minor_mode(name)
  local cur_mode = pome.get_mode()
  if not pome.is_minor_mode(cur_mode) then
    pome.save_mode(cur_mode)
  end
  safe_set_mode(name)
end

prefer_x, _ = pome.get_cursor_pos(); 

function move_cursor_to(x, y)
	if not x or not y then return end
		
	local line_end = pome.get_line_end(y);
	if x > line_end then x = line_end end
		
	pome.move_cursor_to(x,y);
end

function cursor_up()
	local _, y = pome.get_cursor_pos();
	move_cursor_to(prefer_x, y-1);
end

function cursor_down()
	local _, y = pome.get_cursor_pos();
	move_cursor_to(prefer_x, y+1);
end

function cursor_left()
	pome.move_cursor(direction.left, 1);
	prefer_x, _ = pome.get_cursor_pos();
end

function cursor_right()
	pome.move_cursor(direction.right, 1);
	prefer_x, _ = pome.get_cursor_pos();
end

function insert_newline() pome.insert_newlines(1) end

function cursor_line_start()
	local _,y = pome.get_cursor_pos()
  pome.move_cursor_to(0, y)
  prefer_x = 0;
end

function cursor_line_end()
	local _, y = pome.get_cursor_pos();
	local line_end = pome.get_line_end(y);
	move_cursor_to(line_end, y);
	prefer_x = line_end;
end

function delete_line()
  cursor_line_start();

  local _, y = pome.get_cursor_pos();

  pome.set_anchor(0, y + 1);
  pome.delete_selected();
  pome.clear_anchor();
end

local whitespace = make_charset(' \r\t\n')
local word_chars = make_charset("abcdefghijklmnopqrstuvwxyz")
local WORD_chars = make_charset("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
local numbers = make_charset("0123456789")
local underscore = make_charset('_') 

local identifier = merge_sets(word_chars, WORD_chars, numbers);

local function pos_prev(x, y)
	if not x or not y then return end
		
	if x > 0 then return x-1,y end
	if y == 0 then return nil end
		
	local prev_x = pome.get_line_end(y-1)
	return prev_x, y-1
end

local function pos_next(x, y)
	local line_end = pome.get_line_end(y)
	if x < line_end then return x+1, y end
		
	local total_line = pome.get_total_lines()
	if y+1 < total_line then return 0, y+1 end

	return nil 
end

local non_word = merge_sets(identifier, whitespace)

function word_forward()
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
    prefer_x = nx
    return
  else
    nx, ny = pome.forward_match_set(non_word, x, y)
  end

  if nx == nil then return end

  nx, ny = pome.forward_match_notset(whitespace, nx, ny)
  if nx == nil then return end

  move_cursor_to(nx, ny)
  prefer_x = nx
end

function word_backward()
  local x, y = pome.get_cursor_pos()

  local nx, ny = pos_prev(x, y)
  if nx == nil then return end

  -- skip whitespace backward to reach end of previous word
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
  prefer_x = nx
end

function goto_lastline()
	move_cursor_to(prefer_x, pome.get_total_lines() - 1);
end

function goto_firstline()
	move_cursor_to(prefer_x, 0);
end
---@meta

---@class pome_api
---@field move_cursor          fun(dir: string, times: integer)
---@field move_cursor_to       fun(x: integer, y: integer)
---@field quit_editor          fun()
---@field insert_char          fun(ch: integer)
---@field set_mode             fun(mode_name: string)
---@field insert_newlines      fun(times: integer)
---@field save_mode            fun(mode_name: string)
---@field is_minor_mode        fun(mode_name: string): boolean
---@field set_anchor           fun(x: integer?, y: integer?)
---@field clear_anchor         fun()
---@field delete_selected      fun()
---@field get_cursor_pos       fun(): integer, integer
---@field get_line_end         fun(line: integer?): integer?
---@field char_at              fun(x: integer?, y: integer?): integer?
---@field get_total_lines      fun(): integer
---@field get_current_mode     fun(): string?
---@field get_saved_mode       fun(): string?
---@field restore_mode         fun()
---@field call_mode_hook       fun(mode_name: string?, hook_name: string?): boolean
---@field delete_after         fun()
---@field delete_before        fun()
---@field save_file            fun()
---@field load_config          fun(filename: string?)
---@field set_filename         fun(filename: string)
---@field get_filename         fun(): string?
---@field get_config_dir       fun(): string
---@field set_config_dir       fun(path: string)
---@field forward_match        fun(matcher: integer, from_x: integer?, from_y: integer?): integer?, integer?
---@field backward_match       fun(matcher: integer, from_x: integer?, from_y: integer?): integer?, integer?
---@field forward_match_set    fun(charset: table, from_x: integer?, from_y: integer?): integer?, integer?
---@field backward_match_set   fun(charset: table, from_x: integer?, from_y: integer?): integer?, integer?
---@field forward_match_notset fun(charset: table, from_x: integer?, from_y: integer?): integer?, integer?
---@field backward_match_notset fun(charset: table, from_x: integer?, from_y: integer?): integer?, integer?
pome = {}

--- Move the cursor by a given number of steps in a direction.
---@param dir string Direction: "left", "right", "up", "down"
---@param times integer Number of steps (clamped, must be >= 0)
function pome.move_cursor(dir, times) end

--- Move the cursor to absolute byte coordinates (0-based).
---@param x integer column (byte offset within line)
---@param y integer line index
function pome.move_cursor_to(x, y) end

--- Exit the editor (sets running flag to false).
function pome.quit_editor() end

--- Insert a single char at the cursor.
---@param ch string char to insert
function pome.insert_char(ch) end

--- Switch to a named editor mode.
---@param mode_name string mode identifier (e.g. "normal", "insert")
function pome.set_mode(mode_name) end

--- Store the current mode as the "previous" mode (used by minor modes).
---@param mode_name string mode name to save
function pome.save_mode(mode_name) end

--- Check whether a mode is marked as a minor mode.
---@param mode_name string
---@return boolean
function pome.is_minor_mode(mode_name) end

--- Start a selection at an optional position; missing coordinates default to current cursor.
---@param x integer? column (byte offset)
---@param y integer? line index
function pome.set_anchor(x, y) end

--- End the current selection.
function pome.clear_anchor() end

--- Delete the currently selected text range.
function pome.delete_selected() end

--- Return the current cursor position.
---@return integer x column (byte offset)
---@return integer y line index
function pome.get_cursor_pos() end

--- Get the length (in bytes) of a given line.
---@param line integer? line index (defaults to current line); negative returns nil
---@return integer? line_length number of bytes, or nil if line is out of range
function pome.get_line_end(line) end

--- Get the byte at a given position, or nil if out of bounds.
---@param x integer? column (default current)
---@param y integer? line (default current)
---@return string? char or nil
function pome.char_at(x, y) end

--- Return the highest line index (total number of newline-separated lines minus one).
---@return integer
function pome.get_total_lines() end

--- Get the name of the current mode, or nil.
---@return string?
function pome.get_current_mode() end

--- Get the previously saved mode name, or nil.
---@return string?
function pome.get_saved_mode() end

--- Restore the previously saved mode (pop from minor mode).
function pome.restore_mode() end

--- Call a named hook function inside a mode’s Lua table.
---@param mode_name string? mode name; if nil, returns false immediately
---@param hook_name string? hook name; if nil, returns false immediately
---@return boolean true if the call was attempted (even if the hook function errors)
function pome.call_mode_hook(mode_name, hook_name) end

--- Delete the character after the cursor.
function pome.delete_after() end

--- Delete the character before the cursor.
function pome.delete_before() end

--- Save the current buffer to the associated filename.
function pome.save_file() end

--- Execute a Lua configuration file.
---@param filename string? path to config file; defaults to the built-in config file
function pome.load_config(filename) end

--- Set the file path associated with the editor.
---@param filename string
function pome.set_filename(filename) end

--- Get the current file path, or nil if none.
---@return string?
function pome.get_filename() end

--- Return the current configuration directory path.
---@return string
function pome.get_config_dir() end

--- Set the configuration directory path.
---@param path string
function pome.set_config_dir(path) end

--- Search forward for a specific byte character.
---@param matcher integer byte to find
---@param from_x integer? starting column (default cursor)
---@param from_y integer? starting line (default cursor)
---@return integer? x found column, or nil
---@return integer? y found line, or nil
function pome.forward_match(matcher, from_x, from_y) end

--- Search backward for a specific byte character.
---@param matcher integer
---@param from_x integer?
---@param from_y integer?
---@return integer? x
---@return integer? y
function pome.backward_match(matcher, from_x, from_y) end

--- Search forward for a character that belongs to a set (Lua table used as a set of byte keys).
---@param charset table set of bytes
---@param from_x integer? starting column
---@param from_y integer? starting line
---@return integer? x
---@return integer? y
function pome.forward_match_set(charset, from_x, from_y) end

--- Search backward for a character in a set.
---@param charset table
---@param from_x integer?
---@param from_y integer?
---@return integer? x
---@return integer? y
function pome.backward_match_set(charset, from_x, from_y) end

--- Search forward for a character NOT in a set.
---@param charset table
---@param from_x integer?
---@param from_y integer?
---@return integer? x
---@return integer? y
function pome.forward_match_notset(charset, from_x, from_y) end

--- Search backward for a character NOT in a set.
---@param charset table
---@param from_x integer?
---@param from_y integer?
---@return integer? x
---@return integer? y
function pome.backward_match_notset(charset, from_x, from_y) end
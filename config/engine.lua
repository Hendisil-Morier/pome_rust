-- engine.lua  (CORE_LUA)
 
pome.modes = {}
pome.mode_state = pome.mode_state or {
    cur_mode    = nil,
    prev_mode   = nil,
    pending_seq = "",
    change_count = 0,
}

function pome.get_current_mode()
    return pome.mode_state.cur_mode
end

local function get_mode_table(mode_name)
    local t = pome.modes[mode_name]
    return type(t) == "table" and t or nil
end

local function is_in_table(tbl, key)
    for k, _ in pairs(tbl) do
        if type(k) == "string" and #k > #key then
            if k:sub(1, #key) == key and k:sub(#key+1, #key+1) == " " then
                return true
            end
        end
    end
    return false
end

function pome.set_mode(mode_name)
    local old = pome.mode_state.cur_mode
    if old then pome.call_mode_hook(old, "on_exit") end
    pome.mode_state.cur_mode = mode_name
    pome.mode_state.change_count = pome.mode_state.change_count + 1
    pome.mode_state.pending_seq = ""
    local mode_table = get_mode_table(mode_name)
    pome.mode_state.sequences = mode_table and mode_table.sequences or nil
    pome.call_mode_hook(mode_name, "on_enter")
end

function pome.save_mode(mode_name)
    pome.mode_state.prev_mode = mode_name
end

function pome.restore_mode()
    local prev = pome.mode_state.prev_mode
    if prev then
        pome.mode_state.prev_mode = nil
        pome.set_mode(prev)
    end
end

function pome.is_minor_mode(mode_name)
    local mt = get_mode_table(mode_name)
    return mt and not not mt.minor
end

function pome.call_mode_hook(mode_name, hook_name)
    local mt = get_mode_table(mode_name)
    if mt and type(mt[hook_name]) == "function" then
        pcall(mt[hook_name])
    end
end

function pome.call_keymap(key_str)
    local cur = pome.mode_state.cur_mode
    if not cur then return false end
    local mt = get_mode_table(cur)
    if not mt then return false end
    local km = mt.keymap
    if type(km) ~= "table" then return false end
    local fn = km[key_str]
    if type(fn) ~= "function" then return false end
    local ok, _ = pcall(fn)
    return ok
end

function pome.call_default(ch)
    local cur = pome.mode_state.cur_mode
    if not cur then return false end
    local mt = get_mode_table(cur)
    if not mt then return false end
    local df = mt.default
    if type(df) ~= "function" then return false end
    local ok, _ = pcall(df, ch)
    return ok
end

function pome.process_sequences(key_str)
    local key_seqs = (key_str == " ") and "space" or key_str
    local state = pome.mode_state
    local sequences = state.sequences
    if type(sequences) ~= "table" then
        state.pending_seq = ""
        return false
    end
    if state.pending_seq == "" then
        state.pending_seq = key_seqs
    else
        state.pending_seq = state.pending_seq .. " " .. key_seqs
    end
    local pending = state.pending_seq
    local fn = sequences[pending]
    if type(fn) == "function" then
        if is_in_table(sequences, pending) then
            return true
        else
            pcall(fn)
            state.pending_seq = ""
            return true
        end
    else
        if is_in_table(sequences, pending) then
            return true
        else
            state.pending_seq = ""
            return false
        end
    end
end

function pome.dispatch_key(key_str)
    local change_before = pome.mode_state.change_count
    local handled = pome.process_sequences(key_str)
    if not handled then handled = pome.call_keymap(key_str) end
    if not handled then handled = pome.call_default(key_str) end
    local unchanged = (change_before == pome.mode_state.change_count)
    if unchanged and pome.mode_state.cur_mode then
        local cur_minor = pome.is_minor_mode(pome.mode_state.cur_mode)
        local prev = pome.mode_state.prev_mode
        local saved_major = prev and not pome.is_minor_mode(prev)
        if cur_minor and saved_major then
            pome.restore_mode()
        end
    end
end

function pome.statusline()
    local mode = pome.mode_state.cur_mode or "?"
    local fname = pome.get_filename() or "[No Name]"
    local x, y = pome.get_cursor_pos()
    local ln, col = (y or 0) + 1, (x or 0) + 1
    return string.format(" %s | %s Ln %d, Col %d ", mode, fname, ln, col)
end

function pome.main()
  pome.update_scroll()
  pome.render()
  while pome.is_running() do
      local ok, err = pcall(function()
          local key = pome.next_key()
          if key then
              pome.dispatch_key(key)
          end
          pome.update_scroll()
          pome.render()
      end)
      if not ok then
          pome.statusline = function() return "ERROR: " .. tostring(err) end
          pome.render()
          pome.next_key()
          pome.statusline = nil
      end
  end
end
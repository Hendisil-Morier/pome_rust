-- core/modes.lua

pome.modes = {}
pome.mode_state = {
    cur_mode    = nil,
    prev_mode   = nil,
    pending_seq = "",
    change_count = 0,
}

function pome.get_current_mode()
    return pome.mode_state.cur_mode
end

function pome.get_mode_table(mode_name)
    local t = pome.modes[mode_name]
    if type(t) == "table" then return t end
    return nil
end

function pome.set_mode(mode_name)
    local old = pome.mode_state.cur_mode
    if old then pome.call_mode_hook(old, "on_exit") end
    
    pome.mode_state.cur_mode = mode_name
    pome.mode_state.change_count = pome.mode_state.change_count + 1
    pome.mode_state.pending_seq = ""
    
    local mode_table = pome.get_mode_table(mode_name)
    pome.mode_state.sequences = mode_table and mode_table.sequences or nil
    
    pome.call_mode_hook(mode_name, "on_enter")
end

function pome.save_mode(mode_name)
    pome.mode_state.prev_mode = mode_name
end

function pome.restore_mode()
    local prev = pome.mode_state.prev_mode
    if not prev then return end
    
    pome.mode_state.prev_mode = nil
    pome.set_mode(prev)
end

function pome.is_minor_mode(mode_name)
    local mt = pome.get_mode_table(mode_name)
    return mt and not not mt.minor
end

function pome.call_mode_hook(mode_name, hook_name)
    local mt = pome.get_mode_table(mode_name)
    if not mt then return end
    
    local fn = mt[hook_name]
    if type(fn) ~= "function" then return end
    
    pcall(fn)
end

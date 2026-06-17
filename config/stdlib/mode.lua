-- stdlib/mode.lua
-- Mode switching helpers

local M = {}

-- Switch to a mode only if we're not already in it
-- Avoids mode re‑enter side‑effects (like resetting anchor)
function M.safe_set_mode(name)
  if name ~= pome.get_current_mode() then
    pome.set_mode(name)
  end
end

-- Enter a minor (pending) mode: saves the current mode if it's not already a minor mode,
-- then switches to the given minor mode.
function M.enter_minor_mode(name)
  local cur_mode = pome.get_current_mode()
  if not pome.is_minor_mode(cur_mode) then
    pome.save_mode(cur_mode)
  end
  M.safe_set_mode(name)
end

return M
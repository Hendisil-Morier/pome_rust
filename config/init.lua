local config_dir = pome.get_config_dir()
package.path = config_dir .. "/?.lua;" .. config_dir .. "/?/init.lua;" .. package.path

require("runtime")

-- Hot‑reload stdlib and modes on every config reload
for k in pairs(package.loaded) do
  if k:find("^stdlib") or k:find("^modes") then
    package.loaded[k] = nil
  end
end

local std = require("stdlib")

-- Load and register all editing modes
require("modes")

-- Start in normal mode
std.safe_set_mode("normal")
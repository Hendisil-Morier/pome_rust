require("config.runtime")

local script_dir = (...):match("(.*[/\\])") or ""
package.path = script_dir .. "?.lua;" ..
               script_dir .. "?/init.lua;" ..
               package.path

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
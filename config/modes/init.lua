-- modes/init.lua
-- Assumes runtime and stdlib are already loaded; globals (define_mode, bind, etc.) exist.

local normal    = require("modes.normal")
local g_pending = require("modes.g_pending")
local d_pending = require("modes.d_pending")
local insert    = require("modes.insert")
local visual    = require("modes.visual")

-- Build configs by calling the factories
local normal_config    = normal()
local g_pending_config = g_pending()
local d_pending_config = d_pending()
local insert_config    = insert()
local visual_config    = visual()

-- Register modes with the core
define_mode("normal",    normal_config)
define_mode("g_pending", g_pending_config)
define_mode("d_pending", d_pending_config)
define_mode("insert",    insert_config)
define_mode("visual",    visual_config)
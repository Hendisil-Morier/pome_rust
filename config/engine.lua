-- engine.lua

require("core.modes")
require("core.keys")

local layout = require("stdlib.layout")

local row_offset = 0

function pome.render()
    local w, h = pome.get_term_size()
    local screen = { x=0, y=0, width=w, height=h }
    
    local buf_rect, bar_rect = layout.hsplit(screen, h - 1)

    local cx, cy = pome.get_cursor_pos()
    row_offset = layout.compute_scroll(row_offset, cy, buf_rect.height)

    local fname = pome.get_filename() or "[No Name]"
    local mode  = pome.mode_state.cur_mode or "?"
    local status = string.format(" %s | %s | Ln %d, Col %d ", mode, fname, cy+1, cx+1)

    local text_buffer_panel = {
        type = "buffer",
        rect = buf_rect,
        row_offset = row_offset,
        tab_width = 4,
    }

    local status_line_panel = {
        type = "text",
        rect = bar_rect,
        content = status,
        bg = "DarkGray",
        fg = "White",
    }

    pome.draw_panels({ text_buffer_panel, status_line_panel })
end

function pome.main()
    while pome.is_running() do
        pcall(pome.render)

        local ok, err = pcall(function()
            local key = pome.next_key()
            if key then
                pome.dispatch_key(key)
            end
        end)
        
        if not ok then
            local w, h = pome.get_term_size()
            local error_report_panel = {
                type = "text",
                rect = { x=0, y=h-1, width=w, height=1 },
                content = "ERROR: " .. tostring(err),
                bg = "Red",
                fg = "White",
            }
            pome.draw_panels({ error_report_panel })
            pome.next_key()
        end
    end
end
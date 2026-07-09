local M = {}

-- Handles scrolling the buffer if the cursor moves off screen
function M.compute_scroll(row_offset, cursor_y, screen_h)
    local screen_rows = screen_h - 1 -- leave room for status bar
    if cursor_y < row_offset then
        return cursor_y
    elseif cursor_y >= row_offset + screen_rows then
        return cursor_y - screen_rows + 1
    end
    return row_offset
end

-- Splits a rect horizontally: top gets `top_h` rows, bottom gets the rest
function M.hsplit(rect, top_h)
    local top    = { x=rect.x, y=rect.y,         width=rect.width, height=top_h }
    local bottom = { x=rect.x, y=rect.y + top_h, width=rect.width, height=rect.height - top_h }
    return top, bottom
end

-- Splits a rect vertically: left gets `left_w` columns, right gets the rest
function M.vsplit(rect, left_w)
    local left  = { x=rect.x,          y=rect.y, width=left_w,           height=rect.height }
    local right = { x=rect.x + left_w, y=rect.y, width=rect.width - left_w,  height=rect.height }
    return left, right
end

return M

//TODO: rewrite the entire thing once the panel system mature

use ratatui::Frame;
use ratatui::layout::Position as RatPosition;
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use crate::data_types::Editor;

const TAB_WIDTH: usize = 2;

pub fn render(frame: &mut Frame, editor: &Editor)
{
    render_buffer(frame, editor);
    render_status_bar(frame, editor);
}

/// Compute the screen column of the cursor by expanding tabs
fn visual_x(editor: &Editor, cursor_abs: usize, cursor_y: usize) -> usize
{
    // Get the start of the line the cursor is on
    let line_start = editor.buffer.line_to_char(cursor_y);

    let mut col = 0;
    for i in line_start..cursor_abs {
        let byte = editor.buffer.get_char(i);
        match byte {
            Some('\t') => {
                // Advance to next tab stop
                col = (col / TAB_WIDTH + 1) * TAB_WIDTH;
            },
            Some('\n') => {
                // Should not happen because cursor_abs points to somewhere on this line
                // (the loop stops before cursor_abs, and if cursor is on newline, cursor_abs
                // would be at the newline; but cursor_y is the line after, so this case is avoided)
                break;
            },
            Some(_) => {
                col += 1;
            },
            None => break, // shouldn't happen either
        }
    }
    col
}

fn render_buffer(frame: &mut Frame, editor: &Editor)
{
    let area = frame.area();
    let screen_rows = area.height as usize - 1;
    let buf = frame.buffer_mut();

    let logic_len = editor.buffer.len_chars();
    let cursor_abs = editor.cur_info.abs_pos;
    
    let anchor;
    let select_start: i64;
    let select_end: i64;
    let has_anchor;

    if let Some(a) = editor.cur_info.anchor
    {
      anchor = a;
      select_start = anchor.min(cursor_abs) as i64;
      select_end   = anchor.max(cursor_abs) as i64;
      has_anchor = true;
    }
    else
    {
      select_end = -1;
      select_start = -1;
      has_anchor = false;
    }
    
    let mut screen_x: usize = 0;
    let mut screen_y: usize = 0;

    for i in 0..logic_len
    {
        let c = match editor.buffer.get_char(i)
        {
            Some(b) => b,
            None    => break,
        };

        if c == '\n'
        {
            screen_y += 1;
            screen_x = 0;
            continue;
        }

        let visible = screen_y >= editor.row_offset
            && screen_y < editor.row_offset + screen_rows;

        if !visible { continue; }

        let in_selection = editor.cur_info.selecting && has_anchor
            && i as i64 >= select_start
            && (i as i64) < select_end;

        let bg = if in_selection { Color::Blue } else { Color::Reset };
        let style = Style::default().bg(bg);

        if c == '\t'
        {
            let next_tab = (screen_x / TAB_WIDTH + 1) * TAB_WIDTH;
            while screen_x < next_tab
            {
                if screen_x < area.width as usize
                {
                    let cell = buf.cell_mut(RatPosition {
                        x: screen_x as u16,
                        y: (screen_y - editor.row_offset) as u16,
                    });

                    if let Some(cell) = cell
                    {
                        cell.set_char(' ');
                        cell.set_style(style);
                    }
                }
                screen_x += 1;
            }
            continue;
        }

        if screen_x < area.width as usize
        {
            let cell = buf.cell_mut(RatPosition {
                x: screen_x as u16,
                y: (screen_y - editor.row_offset) as u16,
            });

            if let Some(cell) = cell
            {
                cell.set_char(c as char);
                cell.set_style(style);
            }
        }

        screen_x += 1;
    }

    // ------- cursor placement (fixed) -------
    let cur_pos = editor.cursor_pos();
    let cursor_visual_x = visual_x(editor, cursor_abs, cur_pos.y);

    frame.set_cursor_position(RatPosition {
        x: cursor_visual_x as u16,
        y: (cur_pos.y - editor.row_offset) as u16,
    });
}

fn render_status_bar(frame: &mut Frame, editor: &Editor)
{
    let area = frame.area();
    let cur_pos = editor.cursor_pos();

    let mode = editor.mode_info.cur_mode
        .as_deref()
        .unwrap_or("---");

    let filename = editor.filename
        .as_ref()
        .and_then(|p| p.to_str())
        .unwrap_or("[no file]");

    let status = format!(
        "{} | {} : {} | file: {}",
        mode,
        cur_pos.y + 1,
        cur_pos.x + 1,
        filename,
    );

    let y = area.height - 1;
    let style = Style::default()
        .fg(Color::Black)
        .bg(Color::White);

    let span = Span::styled(status, style);
    let buf = frame.buffer_mut();

    for (i, c) in span.content.chars().enumerate()
    {
        if i >= area.width as usize { break; }

        let cell = buf.cell_mut(RatPosition { x: i as u16, y });
        if let Some(cell) = cell
        {
            cell.set_char(c);
            cell.set_style(style);
        }
    }
}
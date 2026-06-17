//TODO: rewrite the entire thing once the panel system mature

use ratatui::Frame;
use ratatui::layout::Position as RatPosition;
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use crate::editor::Editor;

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
    let line_start = editor.buffer.get_line_start(cursor_y);

    let mut col = 0;
    for i in line_start..cursor_abs {
        let byte = editor.buffer.char_at(i);
        match byte {
            Some(b'\t') => {
                // Advance to next tab stop
                col = (col / TAB_WIDTH + 1) * TAB_WIDTH;
            },
            Some(b'\n') => {
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

    let logic_len = editor.buffer.logic_len();

    let cursor_abs = editor.buffer.cursor_abspos();
    let select_start = editor.cur_info.anchor.min(cursor_abs);
    let select_end   = editor.cur_info.anchor.max(cursor_abs);

    let mut screen_x: usize = 0;
    let mut screen_y: usize = 0;

    for i in 0..logic_len
    {
        let c = match editor.buffer.char_at(i)
        {
            Some(b) => b,
            None    => break,
        };

        if c == b'\n'
        {
            screen_y += 1;
            screen_x = 0;
            continue;
        }

        let visible = screen_y >= editor.row_offset
            && screen_y < editor.row_offset + screen_rows;

        if !visible { continue; }

        let in_selection = editor.cur_info.selecting
            && i >= select_start
            && i < select_end;

        let bg = if in_selection { Color::Blue } else { Color::Reset };
        let style = Style::default().bg(bg);

        if c == b'\t'
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
    let cur_pos = editor.buffer.cursor_pos();
    let cursor_visual_x = visual_x(editor, cursor_abs, cur_pos.y);

    frame.set_cursor_position(RatPosition {
        x: cursor_visual_x as u16,
        y: (cur_pos.y - editor.row_offset) as u16,
    });
}

fn render_status_bar(frame: &mut Frame, editor: &Editor)
{
    let area = frame.area();
    let cur_pos = editor.buffer.cursor_pos();

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
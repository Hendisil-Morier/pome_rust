use ratatui::{Frame, layout::Rect, style::{Style}, widgets::Paragraph};
use ropey::Rope;

use crate::{data_types::{misc::CursorInfo, render::{Panel, PanelColor}}, render::{helpers::{draw_chars, place_cursor, selection_bound, to_ratatui_color}, structs::DrawContext}};

pub fn render_text(
  frame: &mut Frame,
  rect: Rect,
  content: &str,
  bg: Option<&PanelColor>,
  fg: Option<&PanelColor>,
  cursor: Option<(u16, u16)>,
)
{
  let mut style = Style::default();
  if let Some(c) = bg {style = style.bg(to_ratatui_color(c));}
  if let Some(c) = fg {style = style.fg(to_ratatui_color(c));}
  
  // 1. Clear the area completely so buffer text doesn't show through
  frame.render_widget(ratatui::widgets::Clear, rect);
  
  // 2. Wrap it in a Block with the style so the entire area gets the background color
  let p = Paragraph::new(content)
      .style(style)
      .block(ratatui::widgets::Block::default().style(style));
      
  frame.render_widget(p, rect);
  
  if let Some((cx, cy)) = cursor {
    frame.set_cursor_position((rect.x + cx, rect.y + cy));
  }
}

pub fn render_buffer
(
  frame: &mut Frame,
  ctx: &DrawContext,
  cursor: Option<&CursorInfo>,
)
{
  draw_chars(frame.buffer_mut(), ctx);
  
  if let Some(c) = cursor 
  {
    place_cursor(frame, ctx, c.abs_pos);
  };
}

pub fn render_panels(frame: &mut Frame, panels: &[Panel], rope: &Rope)
{
  for panel in panels
  {
    match panel
    {
      Panel::Buffer { rect, row_offset, cursor, tab_width } =>
      {
        let ctx = DrawContext{
          rect: *rect,
          row_offset: *row_offset,
          tab_width: *tab_width,
          selection: selection_bound(cursor.as_ref()),
          rope
        };
        
        render_buffer(frame, &ctx, cursor.as_ref());
      },
      
      Panel::Text { rect, content, bg, fg, cursor }
      => render_text(frame, *rect, content, bg.as_ref(), fg.as_ref(), *cursor),
    }
  }
}
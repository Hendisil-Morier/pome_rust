use ratatui::{Frame, buffer::Buffer, layout::Position, style::{Color, Style}};
use ropey::Rope;

use crate::{data_types::{misc::CursorInfo, render::PanelColor}, render::structs::DrawContext};

//helpers
pub(crate) fn visible_range(rope: &Rope, row_offset: usize, height: usize)
-> Option<(usize, usize)>
{
  if row_offset >= rope.len_lines() {return None;}
  let result = (rope.line_to_char(row_offset), row_offset + height);
  
  return Some(result);
}

pub(crate) fn draw_chars(buf: &mut Buffer,ctx: &DrawContext)
{
  let (rect, row_offset, tab_width) = 
    (ctx.rect, ctx.row_offset, ctx.tab_width);
  let rope = ctx.rope;
  
  let (start_char, end_line) = match 
    visible_range(rope, row_offset, rect.height as usize)
    {
      Some(r) => r,
      None => return,
    };
  
  let mut screen_x = 0usize;
  let mut screen_y = row_offset;
  
  //paint char and return the new screen_x
  let mut paint_char = |screen_x: usize, screen_y: usize, c: char, style: Style| -> usize
  {
    //abs position of a line to draw
    let draw_y = ctx.rect.y + (screen_y - ctx.row_offset) as u16;
    
    if c == '\t'
    {
      //calculate tab stop, or, where should the next tab be
      // 0 -> next one would be at eg. 4
      // 1 -> still 4
      // 5 -> 8
      let next_tab = (screen_x / tab_width + 1) * tab_width;
      
      let limit = next_tab.min(ctx.rect.width as usize);
      
      for sx in screen_x..limit
      {
        let x = ctx.rect.x + sx as u16;
        let y = draw_y;
        if let Some(cell) = buf.cell_mut(Position{x, y})
        {
          cell.set_char(' ');
          cell.set_style(style);
        }
      }
      
      return next_tab;
    }     
    
    if screen_x < ctx.rect.width as usize
    {
      let x = ctx.rect.x + screen_x as u16;
      let y = draw_y;
      if let Some(cell) = buf.cell_mut(Position{x, y})
      {
        cell.set_char(c);
        cell.set_style(style);
      }
    }
    return screen_x + 1;
  }; 
  
  for (offset, c) in rope.chars_at(start_char).enumerate()
  {
    if screen_y >= end_line {break;}
    
    //since offset is relative to start_char
    //due to how enumerate work, calculate
    //the abs position of c in the rope
    let abs_pos = start_char + offset;
    
    if c == '\n'
    {screen_x = 0; screen_y += 1; continue;}
    if c.is_control() && c != '\t' {continue;}

     
    let style = char_style(ctx.selection, abs_pos);
    screen_x = paint_char(screen_x, screen_y, c, style);
  }
}

fn char_style(selection: Option<(usize, usize)>, abs_pos: usize)
-> Style
{
  let in_sel = match selection
    {
      Some((start, end)) => abs_pos >= start && abs_pos <= end,
      None => false
    };
  
  let color = if in_sel
    { Color::Blue }
  else
    {Color::Reset};
  
  return Style::default().bg(color);
}

pub(crate) fn to_ratatui_color(c: &PanelColor)
-> Color
{
  let result = match c
  {
    PanelColor::Name(c) => *c,
    PanelColor::Rgb(r, g, b) => Color::Rgb(*r, *g, *b),
  };
  
  return result;
}

pub(crate) fn selection_bound(cursor: Option<&CursorInfo>)
-> Option<(usize, usize)>
{
  let c = cursor?;
  
  if !c.selecting {return None;}
  
  let anchor = c.anchor?;
  let result = ( anchor.min(c.abs_pos), anchor.max(c.abs_pos) );
  
  return Some(result);
}

fn visual_x(
  rope: &Rope, cursor_abs: usize, 
  cursor_y: usize, tab_width: usize
)
-> usize
{
  let line_start = rope.line_to_char(cursor_y);
  let mut col = 0;
  
  for i in line_start..cursor_abs
  {
    match rope.get_char(i)
    {
      Some('\t') => col = (col/tab_width + 1) * tab_width,
      Some('\n') => break,
      Some(_) => col += 1,
      None => break,
    }
  }
  
  col
}

pub(crate) fn place_cursor(
  frame: &mut Frame, 
  ctx: &DrawContext, cursor_abs: usize
)
{
  let rope = ctx.rope;
  let cur_y = rope.char_to_line(cursor_abs);
  
  let vx = visual_x(rope, cursor_abs, cur_y, ctx.tab_width);
  
  let x = ctx.rect.x + vx as u16;
  let y = ctx.rect.y + cur_y.saturating_sub(ctx.row_offset) as u16;
  
  frame.set_cursor_position(Position{x, y});
}
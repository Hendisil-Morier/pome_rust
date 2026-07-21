use ratatui::crossterm::{
    cursor::SetCursorStyle,
    event::Event,
};
use mlua::{Lua, LuaSerdeExt};
use crate::data_types::misc::Position;
use crate::data_types::render::Panel;
use crate::helpers::{*};
use crate::render::render::render_panels;
use crate::{get_editor, get_terminal};

pub fn lua_get_line_end(lua: &Lua, lline: Option<i64>)
-> mlua::Result<mlua::Value>
{
  get_editor!(editor from lua);
	
	let line;
	if let Some(l) = lline
	{
		if l < 0 {return Ok(mlua::Value::Nil);}
		
		let max = editor.max_index_lines();
		if l as usize > max {return Ok(mlua::Value::Nil);}
		
		line = l as usize;
	}
	else {line = editor.cursor_pos().y;}
			
	let line_end = editor.line_len(line);
	
	Ok(mlua::Value::Integer(line_end as i64))
}

pub fn lua_char_at(lua: &Lua, (x, y): (Option<usize>, Option<usize>))
-> mlua::Result<Option<char>>
{
  get_editor!(editor from lua);
	let cur_pos = editor.cursor_pos();
	let x = x.unwrap_or(cur_pos.x);
	let y = y.unwrap_or(cur_pos.y);
	
	let abs_pos = editor.repos_to_abspos(Position{ x, y });
	
	return Ok(editor.buffer.get_char(abs_pos));
}

pub fn lua_get_max_line_index(lua: &Lua, _: ())
-> mlua::Result<usize>
{
  get_editor!(editor from lua);
	return Ok(editor.max_index_lines());
}

pub fn lua_next_key(_: &Lua, _: ()) -> mlua::Result<Option<String>>
{
  let event = ratatui::crossterm::event::read()?;
  
  if let Event::Key(k) = event
  {
    if !k.is_press() && !k.is_repeat()
    {return Ok(None);}
    
    return Ok(keyevent_to_string(k.code, k.modifiers))
  }
  
  return Ok(None);
}

pub fn lua_draw_panels(lua: &Lua, panels: mlua::Value) -> mlua::Result<()>
{
  let mut panels: Vec<Panel> = lua.from_value(panels)?;
  
  get_editor!(editor from lua);
  
  for panel in panels.iter_mut()
  {
    if let Panel::Buffer{cursor, ..} = panel
    {
      *cursor = Some(editor.cur_info.clone());
    }
  }
  
  get_terminal!(mut terminal from lua);
  
  let result = terminal.draw(|frame| render_panels(frame, &panels, &editor.buffer));
  
  if let Err(e) = result{eprintln!("render error: {e}");}
  
  return Ok(());
}

pub fn lua_get_term_size(lua: &Lua, _:()) -> mlua::Result<(usize, usize)>
{
  get_terminal!(terminal from lua);
  
  let size = terminal.size()?;
  
  return Ok((size.width as usize, size.height as usize));
}

pub fn lua_set_cursor_shape(_: &Lua, shape: String)
->mlua::Result<()>
{
  let style = match shape.as_str()
    {
      "block" => SetCursorStyle::SteadyBlock,
      "bar" => SetCursorStyle::SteadyBar,
      "underline" => SetCursorStyle::SteadyUnderScore,
      
      "block_blink" => SetCursorStyle::BlinkingBlock,
      "bar_blink" => SetCursorStyle::BlinkingBar,
      "underline_blink" => SetCursorStyle::BlinkingUnderScore,
      
      _ => return Err(mlua::Error::runtime(format!("unknown cursor shape: {shape}"))),
    };
  
  let result = ratatui::crossterm::execute!(std::io::stdout(), style)
    .map_err(|e| mlua::Error::runtime(e.to_string()));
  
  return result;
}

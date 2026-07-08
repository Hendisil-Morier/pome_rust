use crossterm::cursor::SetCursorStyle;
use crossterm::event::Event;
use mlua::Lua;
use crate::data_types::render::RenderView;
use crate::data_types::misc::Position;
use crate::render::render;
use crate::helpers::{*};
use crate::get_editor;

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
  let event = crossterm::event::read()?;
  
  if let Event::Key(k) = event
  {
    if !k.is_press() && !k.is_repeat()
    {return Ok(None);}
    
    return Ok(keyevent_to_string(k.code, k.modifiers))
  }
  
  return Ok(None);
}

pub fn lua_update_scroll(lua: &Lua, _: ()) -> mlua::Result<()>
{
  get_editor!(mut editor from lua);
	let height = editor.terminal.as_ref()
		.ok_or_else(|| mlua::Error::runtime("terminal not initialized"))?
		.size()?
		.height as usize;
	editor.update_scroll(height);
	return Ok(());
}

pub fn lua_render(lua: &Lua, _: ()) -> mlua::Result<()>
{
  get_editor!(mut editor from lua);

	let pome: mlua::Table = lua.globals().get("pome")?;
	let status_line: String = pome.get::<mlua::Function>("statusline")
		.and_then(|f| f.call::<String>(()))
		.unwrap_or_default();

	let cursor_pos = editor.cursor_pos();
	let buffer = &editor.buffer;
	let view = RenderView {
		cursor_abs: editor.cur_info.abs_pos,
		cursor_pos,
		buffer,
		anchor: editor.cur_info.anchor,
		selecting: editor.cur_info.selecting,
		row_offset: editor.row_offset,
		status_line: &status_line,
	};

	let terminal = editor.terminal.as_mut()
		.ok_or_else(|| mlua::Error::runtime("terminal not initialized"))?;

	let result = terminal.draw(|frame| {
		render(frame, &view);
	});

	if let Err(e) = result
	{eprintln!("render error: {e}");}

	return Ok(());
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
  
  let result = crossterm::execute!(std::io::stdout(), style)
    .map_err(|e| mlua::Error::runtime(e.to_string()));
  
  return result;
}

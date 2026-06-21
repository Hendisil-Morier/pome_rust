use std::path::PathBuf;

use crossterm::event::Event;
use mlua::{Lua};
use crate::data_types::RenderView;
use crate::{render::render, file_handling::save_file, data_types::Position};

use crate::helpers::{*};

pub fn lua_move_cursor(lua: &Lua, (dir, times): (String, i64)) -> mlua::Result<()>
{
	if times < 0 {return Ok(());}
	
	let editor = unsafe {get_editor_mut(lua)?};
	let direction = direction_from_str(&dir)?;
	editor.move_cursor(times as usize, direction);
	
	return Ok(());
}	

pub fn lua_move_cursor_to(lua: &Lua, (x, y): (i64, i64))
-> mlua::Result<()>
{
	if x < 0 || y < 0 {return Ok(());}
	
	let editor = unsafe {get_editor_mut(lua)?};
	let abs_pos = editor.repos_to_abspos(Position { x: x as usize, y: y as usize });
	editor.move_cursor_to(abs_pos);
	
	return Ok(());
}

pub fn lua_quit_editor(lua: &Lua, _: ())
-> mlua::Result<()>
{
	let editor = unsafe {get_editor_mut(lua)?};
	editor.quit();
	return Ok(());
}

pub fn lua_insert_char(lua: &Lua,
  (ch, x, y): (char, Option<usize>, Option<usize>)) 
-> mlua::Result<()>
{
	let editor = unsafe {get_editor_mut(lua)?};
	let cur_pos = editor.cursor_pos();
	
	let x = x.unwrap_or(cur_pos.x);
	let y = y.unwrap_or(cur_pos.y);
	
	let pos = Position{x, y};
	
	editor.insert_char_at(ch, pos);
	return Ok(());
}

pub fn lua_insert_string(lua: &Lua,
  (text, x, y): (String, Option<usize>, Option<usize>)) 
-> mlua::Result<()>
{
	let editor = unsafe {get_editor_mut(lua)?};
	let cur_pos = editor.cursor_pos();
	
	let x = x.unwrap_or(cur_pos.x);
	let y = y.unwrap_or(cur_pos.y);
	
	let pos = Position{x, y};
	
	editor.insert_string_at(text, pos);
	return Ok(());
}

pub fn lua_set_anchor(lua: &Lua, (x, y): (Option<usize>, Option<usize>))
-> mlua::Result<()>
{
	let editor = unsafe {get_editor_mut(lua)?};
	
	let cur_pos = editor.cursor_pos();
	let anchor_x = x.unwrap_or(cur_pos.x);
	let anchor_y = y.unwrap_or(cur_pos.y);
	
	let anchor_pos = Position{x: anchor_x, y : anchor_y};
	let abs_pos = editor.repos_to_abspos(anchor_pos);
	editor.set_anchor(abs_pos);
	
	return Ok(());
}

pub fn lua_clear_anchor(lua: &Lua, _: ())
-> mlua::Result<()>
{
	let editor = unsafe {get_editor_mut(lua)?};

	editor.clear_anchor();
	
	return Ok(());
}

pub fn lua_delete_selected(lua: &Lua, _: ())
-> mlua::Result<()>
{
	let editor = unsafe {get_editor_mut(lua)?};
	
	editor.delete_selected();
	
	return Ok(());
}

pub fn lua_get_cursor_pos(lua: &Lua, _: ())
-> mlua::Result<(usize, usize)>
{
	let editor = unsafe {get_editor(lua)?};
	
	let cur_pos = editor.cursor_pos();
	
	return Ok((cur_pos.x, cur_pos.y));
}

pub fn lua_get_line_end(lua: &Lua, lline: Option<i64>)
-> mlua::Result<mlua::Value>
{
	let editor = unsafe {get_editor(lua)?};
	
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
	let editor = unsafe {get_editor(lua)?};
	let cur_pos = editor.cursor_pos();
	let x = x.unwrap_or(cur_pos.x);
	let y = y.unwrap_or(cur_pos.y);
	
	let abs_pos = editor.repos_to_abspos(Position{ x, y });
	
	return Ok(editor.buffer.get_char(abs_pos));
}

pub fn lua_get_max_line_index(lua: &Lua, _: ())
-> mlua::Result<usize>
{
	let editor = unsafe {get_editor(lua)?};
	return Ok(editor.max_index_lines());
}

pub fn lua_delete_after(lua: &Lua, _: ())
-> mlua::Result<()>
{
	let editor = unsafe {get_editor_mut(lua)?};
	editor.delete_after();
	return Ok(());
}

pub fn lua_delete_before(lua: &Lua, _: ())
-> mlua::Result<()>
{
	let editor = unsafe {get_editor_mut(lua)?};
	editor.delete_before();
	return Ok(());
}

pub fn lua_save_file(lua: &Lua, _:()) -> mlua::Result<()>
{
	let editor = unsafe {get_editor(lua)?};
	save_file(editor)?;
	return Ok(());
}

pub fn lua_set_filename(lua: &Lua, filename: String)
-> mlua::Result<()>
{
	let editor = unsafe {get_editor_mut(lua)?};
	editor.filename = Some(PathBuf::from(filename));
	Ok(())
}

pub fn lua_get_filename(lua: &Lua, _: ())
-> mlua::Result<Option<String>>
{
	let editor = unsafe {get_editor(lua)?};

	let result = editor.filename.as_ref()
.map(|p| p.to_string_lossy().to_string());
	
	return Ok(result);
}

pub fn lua_get_config_dir(lua: &Lua, _: ()) -> mlua::Result<String>
{
    let editor = unsafe { get_editor(lua)? };

    return Ok(editor.config_dir.to_string_lossy().to_string());
}

pub fn lua_set_config_dir(lua: &Lua, path: String) -> mlua::Result<()>
{
    let editor = unsafe { get_editor_mut(lua)? };

    editor.config_dir = PathBuf::from(path);

    return Ok(());
}	
pub fn lua_forward_match(lua: &Lua,
	(matcher, from_x, from_y): (char, Option<usize>, Option<usize>))
-> mlua::Result<mlua::MultiValue>
{
	let editor = unsafe {get_editor(lua)?};
	
	let cur_pos = editor.cursor_pos();
	let from_x = from_x.unwrap_or(cur_pos.x);
	let from_y = from_y.unwrap_or(cur_pos.y);
	
	let from_pos = Position {x: from_x, y: from_y};
	let from_abs = editor.repos_to_abspos(from_pos);
	
	match editor.forward_match(from_abs, matcher)
	{
		Some(abs_result) =>
		{
			let pos = editor.abspos_to_repos(abs_result);
			let iter_result = [
				mlua::Value::Integer(pos.x as i64),
				mlua::Value::Integer(pos.y as i64),
			];
			let result = mlua::MultiValue::from_iter(iter_result);
			return Ok(result);
		},

		None =>
		{
			let result = mlua::MultiValue::from_iter([mlua::Value::Nil]);
			return Ok(result);
		}
	}
}

pub fn lua_backward_match(lua: &Lua,
	(matcher, from_x, from_y): (char, Option<usize>, Option<usize>))
-> mlua::Result<mlua::MultiValue>
{
	let editor = unsafe {get_editor(lua)?};
	
	let cur_pos = editor.cursor_pos();
	let from_x = from_x.unwrap_or(cur_pos.x);
	let from_y = from_y.unwrap_or(cur_pos.y);
	
	let from_pos = Position{x: from_x, y: from_y};
	let from_abs = editor.repos_to_abspos(from_pos);
	
	match editor.backward_match(from_abs, matcher)
	{
		Some(abs_result) =>
		{
			let pos = editor.abspos_to_repos(abs_result);
			let iter_result = [
				mlua::Value::Integer(pos.x as i64),
				mlua::Value::Integer(pos.y as i64),
			];
			let result = mlua::MultiValue::from_iter(iter_result);
			return Ok(result);
		},

		None =>
		{
			let result = mlua::MultiValue::from_iter([mlua::Value::Nil]);
			return Ok(result);
		}
	}
}

pub fn lua_forward_match_set(lua: &Lua,
	(charset, from_x, from_y): (mlua::Table, Option<usize>, Option<usize>))
-> mlua::Result<mlua::MultiValue>
{
	return match_set_impl(lua, (charset, from_x, from_y), true, true);
}

pub fn lua_backward_match_set(lua: &Lua,
	(charset, from_x, from_y): (mlua::Table, Option<usize>, Option<usize>))
-> mlua::Result<mlua::MultiValue>
{
	return match_set_impl(lua, (charset, from_x, from_y), false, true);
}

pub fn lua_forward_match_notset(lua: &Lua,
	(charset, from_x, from_y): (mlua::Table, Option<usize>, Option<usize>))
-> mlua::Result<mlua::MultiValue>
{
	return match_set_impl(lua, (charset, from_x, from_y), true, false);
}

pub fn lua_backward_match_notset(lua: &Lua,
	(charset, from_x, from_y): (mlua::Table, Option<usize>, Option<usize>))
-> mlua::Result<mlua::MultiValue>
{
	return match_set_impl(lua, (charset, from_x, from_y), false, false);
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

pub fn lua_is_running(lua: &Lua, _: ())
-> mlua::Result<bool>
{
  let editor = unsafe{get_editor(lua)?};
  
  Ok(editor.running)
}

pub fn lua_update_scroll(lua: &Lua, _: ()) -> mlua::Result<()>
{
	let editor = unsafe {get_editor_mut(lua)?};
	let height = editor.terminal.as_ref()
		.ok_or_else(|| mlua::Error::runtime("terminal not initialized"))?
		.size()?
		.height as usize;
	editor.update_scroll(height);
	return Ok(());
}

pub fn lua_render(lua: &Lua, _: ()) -> mlua::Result<()>
{
	let editor = unsafe {get_editor_mut(lua)?};

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

pub fn lua_undo(lua: &Lua, _: ())
-> mlua::Result<()>
{
  let editor = unsafe {get_editor_mut(lua)?};
  
  editor.undo();
  
  Ok(())
}

pub fn lua_redo(lua: &Lua, _: ())
-> mlua::Result<()>
{
  let editor = unsafe {get_editor_mut(lua)?};
  
  editor.redo();
  
  Ok(())
}

pub fn lua_begin_undo_group(lua: &Lua, _: ())
-> mlua::Result<bool>
{
  let editor = unsafe{get_editor_mut(lua)?};
  
  let result = editor.history.begin_group();
  
  Ok(result)
}

pub fn lua_end_undo_group(lua: &Lua, _: ())
->mlua::Result<bool>
{
  let editor = unsafe{get_editor_mut(lua)?};
  
  let result = editor.history.end_group();
  
  Ok(result)
}

pub fn lua_can_undo(lua: &Lua, _:())
->mlua::Result<bool>
{
  let editor = unsafe{get_editor(lua)?};
  
  let result = editor.history.can_undo();
  
  Ok(result)
}

pub fn lua_can_redo(lua: &Lua, _:())
->mlua::Result<bool>
{
  let editor = unsafe{get_editor(lua)?};
  
  let result = editor.history.can_redo();
  
  Ok(result)
}
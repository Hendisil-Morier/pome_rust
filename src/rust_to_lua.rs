mod helpers
{
	use crate::{editor::Editor, gap_buffer::{Direction, Position}};
	use mlua::Lua;
	
	pub unsafe fn get_editor(lua: &Lua) -> mlua::Result<&mut Editor>
	{
		let tmp = lua.app_data_ref::<*mut Editor>();
		
		if let Some(s) = tmp
		{
			unsafe {return Ok(&mut **s);}
		}
		
		return Err(mlua::Error::runtime("no editor found in registry"));
	}
	
	pub fn direction_from_str(s: &str) -> mlua::Result<Direction>
	{
		match s
		{
			"left" => Ok(Direction::Left),
			"right" => Ok(Direction::Right),
			"up" => Ok(Direction::Up),
			"down" => Ok(Direction::Down),
			_ => Err(mlua::Error::runtime(format!("unkown direction: {s}")))
		}
	}
	
	pub fn match_set_impl(lua: &Lua,
		(charset, from_x, from_y): (mlua::Table, Option<usize>, Option<usize>),
		forward: bool, want_in_set: bool)
	-> mlua::Result<mlua::MultiValue>
	{
		let editor = unsafe {get_editor(lua)?};
		let cur_pos = editor.buffer.cursor_pos();
		
		let from_x = from_x.unwrap_or(cur_pos.x);
		let from_y = from_y.unwrap_or(cur_pos.y);
		let result_pos;
		
		let from_abs = editor.buffer.repos_to_abspos(Position::new(from_x, from_y));
		
		let logic_len = editor.buffer.logic_len();
		let mut i = from_abs;
		loop
		{
			let cond;
			if (forward)
			{cond = i >= logic_len;}
			else
			{cond = i == usize::MAX;}
			
			if (cond) {break;}
			
			let c;
			if let Some(t) = editor.buffer.char_at(i)
			{c = t;}
			else {break;}
			
			let in_set = charset.contains_key(c)?;
			
			if (in_set == want_in_set)
			{
				result_pos = editor.buffer.abspos_to_repos(i);
				let result = [
				mlua::Value::Integer(result_pos.x as i64),
				mlua::Value::Integer(result_pos.y as i64),
			  ];
				return Ok(mlua::MultiValue::from_iter(result));
			}
			
			if (forward) {i+=1;}
			else
			{
				if (i == 0) {break;}
				i-=1;
			}
		}
		
		let result = [mlua::Value::Nil];
		return Ok(mlua::MultiValue::from_iter(result));
	}
}

pub mod api
{
	use std::path::PathBuf;

use mlua::{Lua, Value::Nil};
	use crate::{file_handling::save_file, gap_buffer::Position};

	use super::helpers::*;
	
	pub fn lua_move_cursor(lua: &Lua, (dir, times): (String, usize)) -> mlua::Result<()>
	{
		let editor = unsafe {get_editor(lua)?};
		let direction = direction_from_str(&dir)?;
		editor.buffer.move_gap(times, direction);
		
		return Ok(());
	}	
	
	pub fn lua_move_cursor_to(lua: &Lua, (x, y): (usize, usize))
	-> mlua::Result<()>
	{
		let editor = unsafe {get_editor(lua)?};
		let abs_pos = editor.buffer.repos_to_abspos(Position { x, y });
		editor.buffer.move_gap_to(abs_pos);
		
		return Ok(());
	}
	
	pub fn lua_quit_editor(lua: &Lua, _: ())
	-> mlua::Result<()>
	{
		let editor = unsafe {get_editor(lua)?};
		editor.quit();
		return Ok(());
	}
	
	pub fn lua_insert_char(lua: &Lua, ch: u8) 
	-> mlua::Result<()>
	{
		let editor = unsafe {get_editor(lua)?};
		editor.buffer.insert(ch);
		return Ok(());
	}
	
	pub fn lua_set_mode(lua: &Lua, mode_name: String)
	-> mlua::Result<()>
	{
		let editor = unsafe {get_editor(lua)?};
		editor.set_mode(&mode_name);
		return Ok(());
	}
	
	pub fn lua_insert_newline(lua: &Lua, times: usize)
	-> mlua::Result<()>
	{
		let editor = unsafe {get_editor(lua)?};
		
		for _ in 0..times
		{editor.buffer.insert(b'\n');}
		
		return Ok(());
	}
	
	pub fn lua_save_mode(lua: &Lua, mode_name: String)
	-> mlua::Result<()>
	{
		let editor = unsafe {get_editor(lua)?};
		
		editor.save_mode(&mode_name);
		
		return Ok(());
	}
	
	pub fn lua_is_minor_mode(lua: &Lua, mode_name: String)
	-> mlua::Result<bool>
	{
		let editor = unsafe {get_editor(lua)?};
		
		let result = editor.is_minor_mode(&mode_name);
		
		return Ok(result);
	}
	
	pub fn lua_set_anchor(lua: &Lua, (x, y): (Option<usize>, Option<usize>))
	-> mlua::Result<()>
	{
		let editor = unsafe {get_editor(lua)?};
		
		let cur_pos = editor.buffer.cursor_pos();
		let anchor_x = x.unwrap_or(cur_pos.x);
		let anchor_y = y.unwrap_or(cur_pos.y);
		
		let anchor_pos = Position::new(anchor_x, anchor_y);
		let abs_pos = editor.buffer.repos_to_abspos(anchor_pos);
		editor.set_anchor(abs_pos);
		
		return Ok(());
	}
	
	pub fn lua_clear_anchor(lua: &Lua, _: ())
	-> mlua::Result<()>
	{
		let editor = unsafe {get_editor(lua)?};

		editor.clear_anchor();
		
		return Ok(());
	}
	
	pub fn lua_delete_selected(lua: &Lua, _: ())
	-> mlua::Result<()>
	{
		let editor = unsafe {get_editor(lua)?};
		
		editor.delete_selected();
		
		return Ok(());
	}
	
	pub fn lua_get_cursor_pos(lua: &Lua, _: ())
	-> mlua::Result<(usize, usize)>
	{
		let editor = unsafe {get_editor(lua)?};
		
		let cur_pos = editor.buffer.cursor_pos();
		
		return Ok((cur_pos.x, cur_pos.y));
	}
	
	pub fn lua_get_line_end(lua: &Lua, line: Option<usize>)
	-> mlua::Result<(usize)>
	{
		let editor = unsafe {get_editor(lua)?};
		
		let target = line.unwrap_or(editor.buffer.cursor_pos().y);
		let line_end = editor.buffer.get_line_length(target);
		
		return Ok((line_end));
	}
	
	pub fn lua_char_at(lua: &Lua, (x, y): (Option<usize>, Option<usize>))
	-> mlua::Result<Option<u8>>
	{
		let editor = unsafe {get_editor(lua)?};
		let cur_pos = editor.buffer.cursor_pos();
		let x = x.unwrap_or(cur_pos.x);
		let y = y.unwrap_or(cur_pos.y);
		
		let abs_pos = editor.buffer.repos_to_abspos(Position{ x, y });
		
		return Ok(editor.buffer.char_at(abs_pos));
	}
	
	pub fn lua_get_max_line_index(lua: &Lua, _: ())
	-> mlua::Result<usize>
	{
		let editor = unsafe {get_editor(lua)?};
		return Ok(editor.buffer.max_line_index());
	}
	
	pub fn lua_get_current_mode(lua: &Lua, _: ())
	-> mlua::Result<Option<String>>
	{
		let editor = unsafe {get_editor(lua)?};
		return Ok(editor.mode_info.cur_mode.clone());
	}
	
	pub fn lua_get_saved_mode(lua: &Lua, _:())
	-> mlua::Result<Option<String>>
	{
		let editor = unsafe {get_editor(lua)?};
		return Ok(editor.mode_info.prev_mode.clone());
	}
	
	pub fn lua_restore_mode(lua: &Lua, _: ())
	-> mlua::Result<()>
	{
		let editor = unsafe {get_editor(lua)?};
		editor.restore_mode();
		return Ok(());
	}
	
	pub fn lua_call_mode_hook(lua: &Lua, (mode_name, hook_name): (String, String))
	-> mlua::Result<()>
	{
		let editor = unsafe {get_editor(lua)?};
		editor.call_mode_hook(&mode_name, &hook_name);
		return Ok(());
	}
	
	pub fn lua_delete_after(lua: &Lua, _: ())
	-> mlua::Result<()>
	{
		let editor = unsafe {get_editor(lua)?};
		editor.buffer.delete_after();
		return Ok(());
	}
	
	pub fn lua_delete_before(lua: &Lua, _: ())
	-> mlua::Result<()>
	{
		let editor = unsafe {get_editor(lua)?};
		editor.buffer.delete_before();
		return Ok(());
	}
	
	pub fn lua_save_file(lua: &Lua, _:()) -> mlua::Result<()>
	{
		let editor = unsafe {get_editor(lua)?};
		save_file(editor)?;
		return Ok(());
	}
	
	pub fn lua_load_config(lua: &Lua, filename: Option<String>)
	-> mlua::Result<()>
	{
		let editor = unsafe {get_editor(lua)?};
		
		let path = match filename
			{
				Some(f) => f,
				None => editor.config_file.to_string_lossy().to_string(),
			};
		
		lua.load(std::path::Path::new(&path)).exec()?;
		
		return Ok(());
	}
	
	pub fn lua_set_filename(lua: &Lua, filename: String)
	-> mlua::Result<()>
	{
		let editor = unsafe {get_editor(lua)?};
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
	    let editor = unsafe { get_editor(lua)? };
	
	    editor.config_dir = PathBuf::from(path);
	
	    return Ok(());
	}	
	pub fn lua_forward_match(lua: &Lua,
		(matcher, from_x, from_y): (u8, Option<usize>, Option<usize>))
	-> mlua::Result<mlua::MultiValue>
	{
		let editor = unsafe {get_editor(lua)?};
		
		let cur_pos = editor.buffer.cursor_pos();
		let from_x = from_x.unwrap_or(cur_pos.x);
		let from_y = from_y.unwrap_or(cur_pos.y);
		
		let from_pos = Position::new(from_x, from_y);
		let from_abs = editor.buffer.repos_to_abspos(from_pos);
		
		match editor.buffer.forward_match(from_abs, matcher)
		{
			Some(abs_result) =>
			{
				let pos = editor.buffer.abspos_to_repos(abs_result);
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
		(matcher, from_x, from_y): (u8, Option<usize>, Option<usize>))
	-> mlua::Result<mlua::MultiValue>
	{
		let editor = unsafe {get_editor(lua)?};
		
		let cur_pos = editor.buffer.cursor_pos();
		let from_x = from_x.unwrap_or(cur_pos.x);
		let from_y = from_y.unwrap_or(cur_pos.y);
		
		let from_pos = Position::new(from_x, from_y);
		let from_abs = editor.buffer.repos_to_abspos(from_pos);
		
		match editor.buffer.backward_match(from_abs, matcher)
		{
			Some(abs_result) =>
			{
				let pos = editor.buffer.abspos_to_repos(abs_result);
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
}

use mlua::Lua;
use crate::data_types::misc::Position;
use crate::get_editor;
use crate::helpers::{*};

pub fn lua_move_cursor(lua: &Lua, (dir, times): (String, i64)) -> mlua::Result<()>
{
	if times < 0 {return Ok(());}
	
	get_editor!(mut editor from lua);
	let direction = direction_from_str(&dir)?;
	editor.move_cursor(times as usize, direction);
	
	return Ok(());
}	

pub fn lua_move_cursor_to(lua: &Lua, (x, y): (i64, i64))
-> mlua::Result<()>
{
	if x < 0 || y < 0 {return Ok(());}
	
	get_editor!(mut editor from lua);
	let abs_pos = editor.repos_to_abspos(Position { x: x as usize, y: y as usize });
	editor.move_cursor_to(abs_pos);
	
	return Ok(());
}

pub fn lua_get_cursor_pos(lua: &Lua, _: ())
-> mlua::Result<(usize, usize)>
{
  get_editor!(mut editor from lua);
	
	let cur_pos = editor.cursor_pos();
	
	return Ok((cur_pos.x, cur_pos.y));
}

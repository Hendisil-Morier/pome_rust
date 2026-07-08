use mlua::Lua;
use crate::data_types::misc::Position;
use crate::helpers::{*};
use crate::get_editor;

pub fn lua_insert_char(lua: &Lua,
  (ch, x, y): (char, Option<usize>, Option<usize>)) 
-> mlua::Result<()>
{
  get_editor!(mut editor from lua);
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
  get_editor!(mut editor from lua);
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
  get_editor!(mut editor from lua);
	
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
  get_editor!(mut editor from lua);

	editor.clear_anchor();
	
	return Ok(());
}

pub fn lua_delete_selected(lua: &Lua, _: ())
-> mlua::Result<()>
{
  get_editor!(mut editor from lua);
	
	editor.delete_selected();
	
	return Ok(());
}

pub fn lua_delete_after(lua: &Lua, _: ())
-> mlua::Result<()>
{
  get_editor!(mut editor from lua);
	editor.delete_after();
	return Ok(());
}

pub fn lua_delete_before(lua: &Lua, _: ())
-> mlua::Result<()>
{
  get_editor!(mut editor from lua);
	editor.delete_before();
	return Ok(());
}

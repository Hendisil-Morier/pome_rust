use std::path::PathBuf;
use mlua::Lua;
use crate::file_handling::save_file;
use crate::helpers::{*};
use crate::get_editor;

pub fn lua_quit_editor(lua: &Lua, _: ())
-> mlua::Result<()>
{
  get_editor!(mut editor from lua);
	editor.quit();
	return Ok(());
}

pub fn lua_save_file(lua: &Lua, _:()) -> mlua::Result<()>
{
  get_editor!(editor from lua);
	save_file(&editor)?;
	return Ok(());
}

pub fn lua_set_filename(lua: &Lua, filename: String)
-> mlua::Result<()>
{
  get_editor!(mut editor from lua);
	editor.filename = Some(PathBuf::from(filename));
	Ok(())
}

pub fn lua_get_filename(lua: &Lua, _: ())
-> mlua::Result<Option<String>>
{
  get_editor!(editor from lua);

	let result = editor.filename.as_ref()
.map(|p| p.to_string_lossy().to_string());
	
	return Ok(result);
}

pub fn lua_get_config_dir(lua: &Lua, _: ()) -> mlua::Result<String>
{
  get_editor!(editor from lua);

    return Ok(editor.config_dir.to_string_lossy().to_string());
}

pub fn lua_set_config_dir(lua: &Lua, path: String) -> mlua::Result<()>
{
  get_editor!(mut editor from lua);

  editor.config_dir = PathBuf::from(path);

  return Ok(());
}	

pub fn lua_is_running(lua: &Lua, _: ())
-> mlua::Result<bool>
{
  get_editor!(editor from lua);
  
  Ok(editor.running)
}

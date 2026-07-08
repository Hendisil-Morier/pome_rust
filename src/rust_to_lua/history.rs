use std::time::Duration;
use mlua::Lua;
use crate::helpers::{*};
use crate::get_editor;

pub fn lua_undo(lua: &Lua, _: ())
-> mlua::Result<()>
{
  get_editor!(mut editor from lua);
  
  editor.undo();
  
  Ok(())
}

pub fn lua_redo(lua: &Lua, _: ())
-> mlua::Result<()>
{
  get_editor!(mut editor from lua);
  
  editor.redo();
  
  Ok(())
}

pub fn lua_begin_undo_group(lua: &Lua, _: ())
-> mlua::Result<bool>
{
  get_editor!(mut editor from lua);
  
  let result = editor.history.begin_group();
  
  Ok(result)
}

pub fn lua_end_undo_group(lua: &Lua, _: ())
->mlua::Result<bool>
{
  get_editor!(mut editor from lua);
  
  let result = editor.history.end_group();
  
  Ok(result)
}

pub fn lua_can_undo(lua: &Lua, _:())
->mlua::Result<bool>
{
  get_editor!(editor from lua);
  
  let result = editor.history.can_undo();
  
  Ok(result)
}

pub fn lua_can_redo(lua: &Lua, _:())
->mlua::Result<bool>
{
  get_editor!(editor from lua);
  
  let result = editor.history.can_redo();
  
  Ok(result)
}

pub fn lua_set_undo_timeout(lua: &Lua, timeout:u64)
->mlua::Result<()>
{
  get_editor!(mut editor from lua);
  
  editor.history.set_group_timeout(Duration::from_millis(timeout));
  
  Ok(())
}

use crate::{editor::Editor, rust_to_lua::api::*};
use mlua::Lua;

pub fn init_lua(editor: &mut Editor) -> mlua::Result<()>
{
	let editor_ptr = editor as *mut Editor;
	
	let lua = &editor.lua;
	
	lua.set_app_data(editor_ptr);
	
	let pome = lua.create_table()?;
	register_primitives(lua, &pome)?;
	
	lua.globals().set("pome", pome)?;
	
	return Ok(());
}

macro_rules! register_primitives {
	($lua:expr, $table:expr, [$(($name:expr, $func:expr)),* $(,)?])
	=> {
		$(
			$table.set($name, $lua.create_function($func)?)?;
		)*
	}
}

fn register_primitives(lua: &Lua, table: &mlua::Table) -> mlua::Result<()>
{
	register_primitives!(lua, table,
		[
			("move_cursor", lua_move_cursor),
		]);

	return Ok(());
}

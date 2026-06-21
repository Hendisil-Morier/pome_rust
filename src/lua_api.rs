use crate::{data_types::Editor, rust_to_lua::*};
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

fn register_primitives(lua: &Lua, table: &mlua::Table) -> mlua::Result<()> {
    register_primitives!(lua, table, [
        ("move_cursor",             lua_move_cursor),
        ("move_cursor_to",          lua_move_cursor_to),
        ("quit_editor",             lua_quit_editor),
        ("insert_char",             lua_insert_char),
        ("delete_after",     lua_delete_after),
        ("delete_before",    lua_delete_before),
        ("set_anchor",              lua_set_anchor),
        ("clear_anchor",            lua_clear_anchor),
        ("delete_selected",         lua_delete_selected),
        ("save_file",               lua_save_file),
        ("get_line_end",            lua_get_line_end),
        ("get_cursor_pos",          lua_get_cursor_pos),
        ("forward_match",           lua_forward_match),
        ("backward_match",          lua_backward_match),
        ("forward_match_set",       lua_forward_match_set),
        ("forward_match_notset",    lua_forward_match_notset),
        ("backward_match_set",      lua_backward_match_set),
        ("backward_match_notset",   lua_backward_match_notset),
        ("char_at",                 lua_char_at),
        ("render",        lua_render),
        ("update_scroll",  lua_update_scroll),
        ("next_key",       lua_next_key),
        ("is_running",     lua_is_running),
        ("get_max_line_index",         lua_get_max_line_index),
        ("set_config_dir",          lua_set_config_dir),
        ("get_config_dir",          lua_get_config_dir),
        ("set_filename", lua_set_filename),
        ("get_filename", lua_get_filename),
    ]);

    return Ok(());
}
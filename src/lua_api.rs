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

fn register_primitives(lua: &Lua, table: &mlua::Table) -> mlua::Result<()> {
    register_primitives!(lua, table, [
        ("move_cursor",             lua_move_cursor),
        ("move_cursor_to",          lua_move_cursor_to),
        ("quit_editor",             lua_quit_editor),
        ("insert_char",             lua_insert_char),
        ("set_mode",                lua_set_mode),
        ("save_mode",               lua_save_mode),
        ("get_current_mode",                lua_get_current_mode),
        ("get_saved_mode",          lua_get_saved_mode),
        ("restore_mode",            lua_restore_mode),
        ("is_minor_mode",           lua_is_minor_mode),
        ("insert_newlines",         lua_insert_newline),
        ("delete_after",     lua_delete_after),
        ("delete_before",    lua_delete_before),
        ("call_mode_hook",          lua_call_mode_hook),
        ("set_anchor",              lua_set_anchor),
        ("clear_anchor",            lua_clear_anchor),
        ("delete_selected",         lua_delete_selected),
        //TODO: ("load_config",             lua_reload_config),
        //TODO: ("save_file",               lua_save_file),
        ("get_line_end",            lua_get_line_end),
        ("get_cursor_pos",          lua_get_cursor_pos),
        ("forward_match",           lua_forward_match),
        ("backward_match",          lua_backward_match),
        ("forward_match_set",       lua_forward_match_set),
        ("forward_match_notset",    lua_forward_match_notset),
        ("backward_match_set",      lua_backward_match_set),
        ("backward_match_notset",   lua_backward_match_notset),
        ("char_at",                 lua_char_at),
        ("get_total_lines",         lua_get_max_line_index),
        //TODO: ("set_config_dir",          lua_set_config_dir),
        //TODO: ("get_config_dir",          lua_get_config_dir),
    ]);

    return Ok(());
}
use std::io;

mod text_buffer;
mod lua_api;
mod rust_to_lua;
mod file_handling;
mod args_handling;
mod render;
mod data_types;
mod history;
mod helpers;

use data_types::editor::Editor;
use lua_api::init_lua;
use file_handling::load_file;
use args_handling::{parse_arguments};

fn main() -> io::Result<()>
{
    let args: Vec<String> = std::env::args().collect();

    let parsed = match parse_arguments(args)
    {
        Ok(p)    => p,
        Err(msg) =>
        {
            eprintln!("error: {}", msg);
            std::process::exit(1);
        }
    };

    let mut editor = Editor::new(
        parsed.filename,
        parsed.config_dir,
        parsed.config_file,
    );

    if let Err(e) = init_lua(&mut editor)
    {
        eprintln!("failed to initialize lua: {e}");
        std::process::exit(1);
    }

    if let Err(e) = editor.lua.load(
        std::path::Path::new(&editor.config_file)).exec()
    {
        eprintln!("failed to load config: {e}");
        std::process::exit(1);
    }

    load_file(&mut editor)?;
    editor.move_cursor_to(0);
    editor.running = true;
    
    editor.terminal = Some(ratatui::init());
    
    let lua = &editor.lua;
    let pome: mlua::Table = lua.globals().get("pome").expect("pome table missing");
    
    //every editing logic will be living in lua
    //the pome.main() function will be the only entry point
    let main_fn: mlua::Function = pome.get("main").expect("pome.main not defined");
    let result = main_fn.call::<()>(());
    
    ratatui::restore();
    
    if let Err(e) = result
    { eprintln!("fatal error in pome.main: {e}"); std::process::exit(1);}
      
    return Ok(());
}

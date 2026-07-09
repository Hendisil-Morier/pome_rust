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

    let editor = Editor::new(
        parsed.filename,
        parsed.config_dir,
    );

    let lua = mlua::Lua::new();

    if let Err(e) = init_lua(&lua)
    {
        eprintln!("failed to initialize lua: {e}");
        std::process::exit(1);
    }

    let terminal = ratatui::init();
    
    let safe_editor = std::rc::Rc::new(std::cell::RefCell::new(editor));
    let safe_term = std::rc::Rc::new(std::cell::RefCell::new(terminal));
    
    lua.set_app_data(safe_editor.clone());
    lua.set_app_data(safe_term);

    if let Err(e) = lua.load(
        std::path::Path::new(&parsed.config_file)).exec()
    {
        eprintln!("failed to load config: {e}");
        ratatui::restore();
        std::process::exit(1);
    }

    {
        let mut e = safe_editor.borrow_mut();
        load_file(&mut e)?;
        e.running = true;
    }
    
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

use std::io;
use crossterm::event::{self, Event};
use ratatui::DefaultTerminal;

mod text_buffer;
mod editor;
mod lua_api;
mod rust_to_lua;
mod file_handling;
mod args_handling;
mod render;

use editor::Editor;
use lua_api::init_lua;
use file_handling::load_file;
use args_handling::{parse_arguments};
use render::render;

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

    let mut terminal = ratatui::init();
    let result = run(&mut terminal, &mut editor);
    ratatui::restore();

    return result;
}

fn run(terminal: &mut DefaultTerminal, editor: &mut Editor) -> io::Result<()>
{
    while editor.running
    {
        terminal.draw(|frame| {
            render(frame, editor);
        })?;

        if let Event::Key(key_event) = event::read()?
        {
            editor.process_key(key_event);
        }

        editor.update_scroll(terminal.size()?.height as usize);
    }

    return Ok(());
}
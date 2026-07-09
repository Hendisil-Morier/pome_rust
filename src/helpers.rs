use std::{cell::RefCell, rc::Rc};

use crate::data_types::{editor::Editor, misc::{Direction}};
use crossterm::event::{KeyCode, KeyModifiers};
use mlua::Lua;
use ratatui::DefaultTerminal;

type SafeEditor = Rc<RefCell<Editor>>;
#[doc(hidden)]
pub fn get_editor_impl(lua: &Lua)
-> mlua::Result<SafeEditor>
{
  let tmp = lua.app_data_ref::<SafeEditor>()
    .ok_or(mlua::Error::runtime("no editor found in registry"))?;
  
  return Ok(tmp.clone());
}

type SafeTerminal = Rc<RefCell<DefaultTerminal>>;
#[doc(hidden)]
pub fn get_terminal_impl(lua: &Lua)
-> mlua::Result<SafeTerminal>
{
  let tmp = lua.app_data_ref::<SafeTerminal>()
    .ok_or(mlua::Error::runtime("no terminal found in registry"))?;
  
  return Ok(tmp.clone());
}

#[macro_export]
macro_rules! get_editor
{
  ($name:ident from $lua:expr) =>
  {
    let __tmp = get_editor_impl($lua).unwrap();
    let $name = __tmp.borrow();
  };
  (mut $name:ident from $lua:expr) =>
  {
    let __tmp = get_editor_impl($lua).unwrap();
    let mut $name = __tmp.borrow_mut();
  }
}

#[macro_export]
macro_rules! get_terminal
{
  ($name:ident from $lua:expr) =>
  {
    let __tmp = get_terminal_impl($lua).unwrap();
    let $name = __tmp.borrow();
  };
  (mut $name:ident from $lua:expr) =>
  {
    let __tmp = get_terminal_impl($lua).unwrap();
    let mut $name = __tmp.borrow_mut();
  }
}

pub fn keyevent_to_string(code: KeyCode, modifiers: KeyModifiers) -> Option<String>
{
    let mut result = String::new();

    if modifiers.contains(KeyModifiers::ALT)
    {
        result.push_str("alt+");
    }
    if modifiers.contains(KeyModifiers::CONTROL)
    {
        result.push_str("ctrl+");
    }
    if modifiers.contains(KeyModifiers::SHIFT)
    {
        result.push_str("shift+");
    }

    match code
    {
        KeyCode::Char(c)   => result.push_str(&c.to_string()),
        KeyCode::Left      => result.push_str("arrow_left"),
        KeyCode::Right     => result.push_str("arrow_right"),
        KeyCode::Up        => result.push_str("arrow_up"),
        KeyCode::Down      => result.push_str("arrow_down"),
        KeyCode::Enter     => result.push_str("enter"),
        KeyCode::Backspace => result.push_str("backspace"),
        KeyCode::Delete    => result.push_str("delete"),
        KeyCode::Esc       => result.push_str("esc"),
        _                  => return None,
    }

    return Some(result);
}

pub fn direction_from_str(s: &str) -> mlua::Result<Direction>
{
	match s
	{
		"left" => Ok(Direction::Left),
		"right" => Ok(Direction::Right),
		"up" => Ok(Direction::Up),
		"down" => Ok(Direction::Down),
		_ => Err(mlua::Error::runtime(format!("unkown direction: {s}")))
	}
}

use crate::data_types::{Editor, Direction, Position};
use crossterm::event::{KeyCode, KeyModifiers};
use mlua::Lua;
	
pub unsafe fn get_editor(lua: &Lua) -> mlua::Result<&Editor>
{
 	let tmp = lua.app_data_ref::<*mut Editor>();
 	if let Some(s) = tmp
 	{ unsafe {return Ok(&**s);} }
 	return Err(mlua::Error::runtime("no editor found in registry"));
}

#[allow(clippy::mut_from_ref)]
pub unsafe fn get_editor_mut(lua: &Lua) -> mlua::Result<&mut Editor>
{
 	let tmp = lua.app_data_ref::<*mut Editor>();
 	if let Some(s) = tmp
 	{ unsafe {return Ok(&mut **s);} }
 	return Err(mlua::Error::runtime("no editor found in registry"));
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

pub fn match_set_impl(lua: &Lua,
	(charset, from_x, from_y): (mlua::Table, Option<usize>, Option<usize>),
	forward: bool, want_in_set: bool)
-> mlua::Result<mlua::MultiValue>
{
	let editor = unsafe {get_editor(lua)?};
	let cur_pos = editor.cursor_pos();
	
	let from_x = from_x.unwrap_or(cur_pos.x);
	let from_y = from_y.unwrap_or(cur_pos.y);
	let result_pos;
	
	let from_abs = editor.repos_to_abspos(Position{x: from_x, y: from_y});
	
	let logic_len = editor.buffer.len_chars();
	let mut i = from_abs;
	loop
	{
		let cond;
		if forward 
		{cond = i >= logic_len;}
		else
		{cond = i == usize::MAX;}
		
		if cond  {break;}
		
		let c;
		if let Some(t) = editor.buffer.get_char(i)
		{c = t;}
		else {break;}
		
		let in_set = charset.contains_key(c)?;
		
		if in_set == want_in_set 
		{
			result_pos = editor.abspos_to_repos(i);
			let result = [
			mlua::Value::Integer(result_pos.x as i64),
			mlua::Value::Integer(result_pos.y as i64),
		  ];
			return Ok(mlua::MultiValue::from_iter(result));
		}
		
		if forward  {i+=1;}
		else
		{
			if i == 0  {break;}
			i-=1;
		}
	}
	
	let result = [mlua::Value::Nil];
	return Ok(mlua::MultiValue::from_iter(result));
}

impl Editor
{
  pub fn max_index_lines(&self) -> usize
  {
    self.buffer.len_lines().saturating_sub(1)
  }
  
  pub fn line_len(&self, target_line: usize) -> usize
  {
    let target_line = self.max_index_lines().min(target_line);
    let line = self.buffer.line(target_line);
    
    line.len_chars()
  }  
  
  pub fn repos_to_abspos(&self, pos: Position) -> usize
  {
    let max_line = self.max_index_lines();
    let y = pos.y.min(max_line);
    
    let line_start = self.buffer.line_to_char(y);
    let x = pos.x.min(self.line_len(y));
    
    line_start + x
  }
  
  pub fn abspos_to_repos(&self, abs_pos: usize) -> Position
  {
    let max_line = self.max_index_lines();
    let abs_pos = self.buffer.len_chars().min(abs_pos);
    
    let y = max_line.min(self.buffer.char_to_line(abs_pos));
    let x = abs_pos - self.buffer.line_to_char(y);
    
    Position { x, y }
  }
  
  pub fn cursor_pos(&self) -> Position
  {
    self.abspos_to_repos(self.cur_info.abs_pos)
  }
}

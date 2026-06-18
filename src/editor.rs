use std::path::PathBuf;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ropey::Rope;

// #[derive(Default, Clone, Copy)]
// pub struct Dimension
// {
// 	pub w: usize,
// 	pub h: usize,
// }

pub struct CursorInfo
{
	pub abs_pos: usize,
	pub anchor: Option<usize>,
	pub selecting: bool,
}

pub struct ModeInfo
{
	pub cur_mode: Option<String>,
	pub prev_mode: Option<String>,
	pub change_count: usize,
}

pub struct Editor
{
	pub filename: Option<PathBuf>,
	pub config_dir: PathBuf,
	pub config_file: PathBuf,
	pub lua: mlua::Lua,

	pub buffer: ropey::Rope,
	// pub dim: Dimension,

	pub mode_info: ModeInfo,
	pub cur_info: CursorInfo,

	pub row_offset: usize,
	pub running: bool,

	// pub panels: Vec<Panel>,
}

//helpers
impl Editor
{
	fn get_mode_table(&self, mode_name: &str) -> Option<mlua::Table>
	{
		let lua = &self.lua;
		let modes;
		
		if let Ok(t) = lua.globals().get::<mlua::Table>("modes")
		{modes = t;}
		else {return None}
		
		let mode_table;
		
		if let Ok(t) = modes.get::<mlua::Table>(mode_name)
		{mode_table = t;}
		else {return None;}
		
		return Some(mode_table);
	}
}

impl Editor
{
	pub fn new(filename: Option<PathBuf>, config_dir: PathBuf, config_file: PathBuf)	
	-> Self
	{
		let lua = mlua::Lua::new();
		let buffer = Rope::new();
		// let dim = Dimension::default();
		let row_offset = 0;
		let running = false;
		
		let mode_info = ModeInfo{
			cur_mode: None,
			prev_mode: None,
			change_count: 0,
		};
		
		let cur_info = CursorInfo{
			abs_pos: 0,
			anchor: None,
			selecting: false,
		};
		
		return Self{
			filename,
			config_dir,
			config_file,
			lua,
			buffer,
			// dim,
			mode_info,
			cur_info,
			row_offset,
			running,
		}
	}
	
	pub fn is_minor_mode(&self, mode_name: &str) -> bool
	{
		let mode_table;
		
		if let Some(t) = self.get_mode_table(mode_name)
		{mode_table = t;}
		else {return false;}
		
		let is_minor = mode_table.get::<bool>("minor");
		return is_minor.unwrap_or(false);
	}
	
	pub fn call_mode_hook(&self, mode_name: &str, hook_name: &str)
	{
		let mode_table;

		if let Some(t) = self.get_mode_table(mode_name)
		{mode_table = t}
		else {return;}
		
		let hook;
		if let Ok(f) = mode_table.get::<mlua::Function>(hook_name)
		{hook = f;}
		else {return;}
		
		if let Err(e) = hook.call::<()>(())
		{eprintln!("Error in {hook_name} hook for mode {mode_name}: {e}");}
	}
	
	pub fn set_mode(&mut self, mode_name: &str)
	{
		self.mode_info.cur_mode = Some(mode_name.to_string());
		self.mode_info.change_count += 1;
	}
	
	pub fn save_mode(&mut self, mode_name: &str)
	{
		self.mode_info.prev_mode = Some(mode_name.to_string());
	}
	
	pub fn restore_mode(&mut self)
	{
		if let Some(mode_name) = self.mode_info.prev_mode.take()
		{
			self.set_mode(&mode_name);
		}
		else {return;}
	}
	
	pub fn call_keymap(&self, key_str: &str) -> bool
	{
		let mode_table: mlua::Table;
		if let Some(s) = &self.mode_info.cur_mode
		{
			if let Some(t) = self.get_mode_table(&s)
			{mode_table = t;}
			else {return false;}
		}
		else {return false;}
		
		let keymap: mlua::Table;
		if let Ok(s) = mode_table.get("keymap")
		{keymap = s;}
		else {return false;}
		
		let func: mlua::Function;
		if let Ok(f) = keymap.get(key_str)
		{func = f;}
		else {return false;}
		
		if let Err(e) = func.call::<()>(())
		{
			eprintln!("Lua keymap error: {e}");
			return false;
		}
		
		return true;
	}
	
	pub fn call_default(&self, ch: &str) -> bool
	{
		let mode_table: mlua::Table;
		if let Some(s) = &self.mode_info.cur_mode
		{
			if let Some(t) = self.get_mode_table(&s)
			{mode_table = t;}
			else {return false;}
		}
		else {return false;}
		
		let default_fn: mlua::Function;
		if let Ok(s) = mode_table.get::<mlua::Function>("default")
		{default_fn = s;}
		else {return false;}
		
		if let Err(e) = default_fn.call::<()>(ch)
		{
			eprintln!("Lua default entry error: {e}");
			return false;
		}
		
		return true;
	}
	
	pub fn quit(&mut self)
	{
		self.running = false;
	}
}

fn keyevent_to_string(code: KeyCode, modifiers: KeyModifiers) -> Option<String>
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

impl Editor
{
	pub fn process_key(&mut self, event: KeyEvent)
	{
		if (!event.is_press() && !event.is_repeat())
		{return;}
		
		let key_str: String;
		
		if let Some(s) = keyevent_to_string(event.code, event.modifiers)
		{key_str = s;}
		else {return;}
		
		let mode_before = self.mode_info.change_count;
		
		let mut handled = self.call_keymap(&key_str);
		if (!handled)
		{handled = self.call_default(&key_str);}
		
		if (!handled) {/*TODO: handling empty mode table*/}
		
		let mode_unchanged = mode_before == self.mode_info.change_count;
		
		let current_is_minor: bool;
		
		if let Some(m) = &self.mode_info.cur_mode
		{
			current_is_minor = self.is_minor_mode(m);
		}
		else {return;}
		
		let saved_is_major: bool;
		
		if let Some(m) = &self.mode_info.prev_mode
		{
			saved_is_major = !self.is_minor_mode(m);
		}
		else {saved_is_major = false;}
		
		if (mode_unchanged && current_is_minor && saved_is_major)
		{self.restore_mode();}
	}
	
	pub fn update_scroll(&mut self, screen_h: usize)
	{
		let cur_pos = self.cursor_pos();
		let screen_rows = screen_h - 1;
		
		if (cur_pos.y < self.row_offset)
		{self.row_offset = cur_pos.y;}
		
		if (cur_pos.y >= self.row_offset + screen_rows)
		{self.row_offset = cur_pos.y - screen_rows + 1;}
	}
	
	pub fn delete_selected(&mut self)
	{
		// self.buffer.delete_selected(self.cur_info.anchor);
		if self.cur_info.selecting == false
		{return;}
		
		let anchor;
		if let Some(a) = self.cur_info.anchor
		{anchor = a;}
		else {return;}
		
		let cur_abs_pos = self.cur_info.abs_pos;
		
		let start = anchor.min(cur_abs_pos);
		let end = anchor.max(cur_abs_pos);
		
		self.buffer.remove(start..end);
	}
	
	pub fn set_anchor(&mut self, abs_pos: usize)
	{
		self.cur_info.anchor = Some(abs_pos);
		self.cur_info.selecting = true;
	}
	
	pub fn clear_anchor(&mut self)
	{
		self.cur_info.selecting = false;
	}
}
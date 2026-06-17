use std::path::PathBuf;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::gap_buffer::{GapBuffer};

#[derive(Default, Clone, Copy)]
pub struct Dimension
{
	pub w: usize,
	pub h: usize,
}

pub struct CursorInfo
{
	// pub pos: Position,
	pub anchor: usize,
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

	pub buffer: GapBuffer,
	pub dim: Dimension,

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
		let buffer = GapBuffer::new();
		let dim = Dimension::default();
		let row_offset = 0;
		let running = false;
		
		let mode_info = ModeInfo{
			cur_mode: None,
			prev_mode: None,
			change_count: 0,
		};
		
		let cur_info = CursorInfo{
			// pos: Position::default(),
			anchor: 0,
			selecting: false,
		};
		
		return Self{
			filename,
			config_dir,
			config_file,
			lua,
			buffer,
			dim,
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

fn keyevent_to_string(code: KeyCode, modifier: KeyModifiers) -> Option<String>
{
	if modifier.contains(KeyModifiers::CONTROL)
	{
		if let KeyCode::Char(c) = code
		{
			return Some(format!("ctrl+{}", c));
		}
	}
	
	let result;
	match code
	{
		KeyCode::Char(c) => result = Some(c.to_string()),
		KeyCode::Left => result = Some("arrow_left".to_string()),
		KeyCode::Right => result = Some("arrow_right".to_string()),
		KeyCode::Up => result = Some("arrow_up".to_string()),
		KeyCode::Down => result = Some("arrow_down".to_string()),
		KeyCode::Enter => result = Some("enter".to_string()),
		KeyCode::Backspace => result = Some("backspace".to_string()),
		KeyCode::Delete => result = Some("delete".to_string()),
		KeyCode::Esc => result = Some("esc".to_string()),
		_ => result = None,
	};
	
	return result;
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
		
		let mode_changed = mode_before == self.mode_info.change_count;
		
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
		
		if (mode_changed && current_is_minor && saved_is_major)
		{self.restore_mode();}
	}
	
	pub fn update_scroll(&mut self, screen_h: usize)
	{
		let cur_pos = self.buffer.cursor_pos();
		let screen_rows = screen_h - 1;
		
		if (cur_pos.y < self.row_offset)
		{self.row_offset = cur_pos.y;}
		
		if (cur_pos.y >= self.row_offset + screen_rows)
		{self.row_offset = cur_pos.y - screen_rows + 1;}
	}
	
	pub fn delete_selected(&mut self)
	{
		self.buffer.delete_selected(self.cur_info.anchor);
	}
	
	pub fn set_anchor(&mut self, abs_pos: usize)
	{
		self.cur_info.anchor = abs_pos;
		self.cur_info.selecting = true;
	}
	
	pub fn clear_anchor(&mut self)
	{
		self.cur_info.selecting = false;
	}
}
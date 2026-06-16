use std::path::PathBuf;

use crate::gap_buffer::{GapBuffer, Position};

#[derive(Default, Clone, Copy)]
pub struct Dimension
{
	pub w: usize,
	pub h: usize,
}

pub struct CursorInfo
{
	pub pos: Position,
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
			pos: Position::default(),
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
		if let Some(cur) = &self.mode_info.cur_mode
		{self.call_mode_hook(mode_name, "on_exit");}
		
		self.mode_info.cur_mode = Some(mode_name.to_string());
		self.mode_info.change_count += 1;
		
		self.call_mode_hook(mode_name, "on_enter");
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
	
	pub fn call_keymap(&mut self, key_str: &str) -> bool
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
	
	pub fn call_default(&self, ch: u32) -> bool
	{
		if (ch == 0) {return false;}
		
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
}
use std::path::PathBuf;
use ropey::Rope;

pub enum Direction
{
	Left,
	Right,
	Up,
	Down,
}

#[derive(Default, Clone, Copy)]
pub struct Position
{
	pub x: usize,
	pub y: usize,
}


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
	
	pub pending_seq: String,
	pub sequences: Option<mlua::Table>,
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

			pending_seq: String::new(),
			sequences: None,
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
}
use std::path::PathBuf;

use ratatui::DefaultTerminal;
use ropey::Rope;

use crate::data_types::{history::History, misc::CursorInfo};

pub struct Editor
{
	pub filename: Option<PathBuf>,
	pub config_dir: PathBuf,
	pub config_file: PathBuf,
	pub lua: mlua::Lua,

	pub buffer: ropey::Rope,
	pub terminal: Option<DefaultTerminal>,
	// pub dim: Dimension,

	pub cur_info: CursorInfo,

	pub row_offset: usize,
	pub running: bool,

	pub(crate) history: History,
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

		// let mode_info = ModeInfo{
		// 	cur_mode: None,
		// 	prev_mode: None,
		// 	change_count: 0,

		// 	pending_seq: String::new(),
		// 	sequences: None,
		// };

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
			terminal: None,
			// dim,
			// mode_info,
			cur_info,
			row_offset,
			running,
			history: History::new(),
		}
	}
}

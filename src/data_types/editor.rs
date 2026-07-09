use std::path::PathBuf;

use ropey::Rope;

use crate::data_types::{history::History, misc::CursorInfo};

pub struct Editor
{
	pub filename: Option<PathBuf>,
	pub config_dir: PathBuf,

	pub buffer: ropey::Rope,

	pub cur_info: CursorInfo,

	pub running: bool,

	pub(crate) history: History,
}

impl Editor
{

	pub fn new(filename: Option<PathBuf>, config_dir: PathBuf)
	-> Self
	{
		let buffer = Rope::new();
		// let dim = Dimension::default();
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
			buffer,
			// dim,
			// mode_info,
			cur_info,
			running,
			history: History::new(),
		}
	}
}

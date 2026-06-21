use std::path::PathBuf;
use ratatui::DefaultTerminal;
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

#[derive(Clone, Debug)]
pub enum Edit
{
  Insert {pos: usize, text: String},
  Delete {pos: usize, text: String},
}

#[derive(Clone, Debug)]
pub struct EditBatch
{
  edits: Vec<Edit>,
  cursor_before: usize,
  cursor_after: usize,
}

pub struct History
{
  batches: Vec<EditBatch>,
  position: usize,
  
  //edit being accumulated, not yet committed to batches  
  current_batch: Option<EditBatch>,
  last_edit_time: std::time::Instant,
  group_timeout: std::time::Duration,

  explicit_group: bool,
}

impl History
{
  pub fn new() -> Self
  {
    Self
    {
      batches: Vec::new(),
      position: 0,
      current_batch: None,
      last_edit_time: std::time::Instant::now(),
      group_timeout: std::time::Duration::from_millis(1000),
      explicit_group: false,
    }
  }
}

pub struct CursorInfo
{
	pub abs_pos: usize,
	pub anchor: Option<usize>,
	pub selecting: bool,
}

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

	history: History,//in construction
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

pub struct RenderView<'a>
{
  pub buffer: &'a ropey::Rope,
  pub cursor_abs: usize,
  pub cursor_pos: Position,
  pub anchor: Option<usize>,
  pub selecting: bool,
  pub row_offset: usize,
  pub status_line: &'a str,
}
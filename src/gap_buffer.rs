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

impl Position
{
	pub fn new(x: usize, y: usize) -> Self
	{
		return Self{
			x, y
		};
	}
}

pub struct GapBuffer
{
  buf: Vec<u8>,
  gap_start: usize,
  gap_end: usize,
}

impl GapBuffer
{
	pub fn new() -> Self
	{
		const init_size: usize = 32;
		Self{
			buf: vec![0u8; init_size],
			gap_start: 0,
			gap_end: init_size,
		}
	}
	
	fn expand(&mut self, new_cap: usize)
	{
		let old_cap = self.buf.len();
		let tail_len = old_cap - self.gap_end;
		let new_gap_end = new_cap - tail_len;
		
		self.buf.resize(new_cap, 0);
		self.buf.copy_within(self.gap_end..old_cap, new_gap_end);
		
		self.gap_end = new_gap_end;
	}
	
	pub fn append(&mut self, ch: u8)
	{
		if (self.gap_start >= self.gap_end)
		{
			self.expand(self.buf.len() * 2);
		}
		
		self.gap_end -= 1;
		self.buf[self.gap_end] = ch;
	}
	
	pub fn insert(&mut self, ch: u8)
	{
		if (self.gap_start >= self.gap_end)
		{
			self.expand(self.buf.len() * 2);
		}
		
		self.buf[self.gap_start] = ch;
		self.gap_start += 1;
	}
	
	pub fn delete_before(&mut self) -> bool
	{
		if (self.gap_start == 0) {return false;}
		
		self.gap_start -= 1;
		return true;
	}
	
	pub fn delete_after(&mut self) -> bool
	{
		if (self.gap_end == self.buf.len()) {return false;}
		
		self.gap_end += 1;
		return true;
	}
	
	pub fn delete_selected(&mut self, anchor: usize)
	{
		let gap_size = self.gap_end - self.gap_start;
		
		if (anchor > self.gap_start)
		{
			self.gap_end = anchor + gap_size;
		}
		else if (anchor < self.gap_start)
		{
			self.gap_start = anchor;
		}
	}
	
	pub fn move_gap_to(&mut self, abs_pos: usize)
	{
		let safe_pos = self.logic_len();
		let abs_pos = abs_pos.min(safe_pos);
		
		if (abs_pos < self.gap_start)
		{
			self.move_gap_horizontal(self.gap_start - abs_pos, Direction::Left);
		}
		else if (abs_pos > self.gap_start)
		{
			self.move_gap_horizontal(abs_pos - self.gap_start, Direction::Right);
		}
	}
	
	pub fn move_gap_horizontal(&mut self, distance: usize, dir: Direction)
	{
		if (distance == 0) {return;}
		
		let safe_distance;
		match dir
		{
			Direction::Right => safe_distance = self.buf.len() - self.gap_end,
			Direction::Left => safe_distance = self.gap_start,
			_ => return,
		};
		
		let distance = distance.min(safe_distance);
		
		match dir
		{
			Direction::Left => {
				let src = self.gap_start - distance;
				let dst = self.gap_end - distance;
				
				self.buf.copy_within(src..self.gap_start, dst);
				
				self.gap_start -= distance;
				self.gap_end -= distance;
			},
			
			Direction::Right => {
				let src = self.gap_end;
				let dst = self.gap_start;

				self.buf.copy_within(src..src + distance, dst);
				
				self.gap_start += distance;
				self.gap_end += distance;
			},
			
			_ => {},
		};
	}
	
	pub fn logic_len(&self) -> usize
	{
		return self.gap_start + (self.buf.len() - self.gap_end);
	}
	
	pub fn char_at(&self, abs_pos: usize) -> Option<u8>
	{
		if (abs_pos >= self.logic_len()) {return None};
		
		let real_pos;
		if (abs_pos >= self.gap_start)
		{
			real_pos = abs_pos + (self.gap_end - self.gap_start);
		}
		else
		{
			real_pos = abs_pos;
		};
		
		return Some(self.buf[real_pos]);
	}
	
	pub fn get_line_start(&self, target_line: usize) -> usize
	{
		let logic_len = self.logic_len();
		let mut lines = 0;
		
		for i in 0..logic_len
		{
			if (lines == target_line) {return i;}
			
			if (self.char_at(i) == Some(b'\n')) {lines += 1;}
		};
		
		return logic_len;
	}
	
	pub fn get_line_length(&self, target_line: usize) -> usize
	{
		let logic_len = self.logic_len();
		let line_start = self.get_line_start(target_line);
		
		let mut i = 0;
		while (line_start + i < logic_len)
		{
			if (self.char_at(line_start + i) == Some(b'\n'))
				{break;}
			i += 1;
		};
		
		return i;
	}
	
	pub fn max_line_index(&self) -> usize
	{
		let mut lines = 0;
		for i in 0..self.logic_len()
		{
			if (self.char_at(i) == Some(b'\n'))
			{lines += 1;}
		};
		
		return lines;
	}
}

impl GapBuffer
{

	pub fn cursor_abspos(&self) -> usize
	{
		return self.gap_start;
	}
	
	pub fn abspos_to_repos(&self, abs_pos: usize) -> Position
	{
		let abs_pos = abs_pos.min(self.logic_len());
		let mut pos = Position::default();
		
		for i in 0..abs_pos
		{
			if (self.char_at(i) == Some(b'\n'))
			{
				pos.x = 0;
				pos.y += 1;
			}
			else {pos.x += 1;}
		};
		
		return pos;
	}
	
	pub fn repos_to_abspos(&self, repos: Position) -> usize
	{
		let max_pos = self.abspos_to_repos(self.logic_len());
		
		let overgrown = repos.y > max_pos.y
			|| (repos.y == max_pos.y && repos.x > max_pos.x);
		
		if (overgrown == true)
		{return self.logic_len();}
		
		let line_len = self.get_line_length(repos.y);
		let clamp_x = repos.x.min(line_len);
		let line_start = self.get_line_start(repos.y);
		
		return line_start + clamp_x;
	}
	
	pub fn cursor_pos(&self) -> Position
	{
		return self.abspos_to_repos(self.gap_start);
	}
	
	pub fn move_gap_vertical(&mut self, target_line: usize)
	{
		let cur_pos = self.cursor_pos();
		let line_length = self.get_line_length(target_line);
		
		let line_start = self.get_line_start(target_line);
		let target_x = cur_pos.x.min(line_length);
		
		self.move_gap_to(line_start + target_x);
	}
	
	pub fn move_gap(&mut self, times: usize, dir: Direction)
	{
		if (times == 0) {return;}
		
		let cur_pos = self.cursor_pos();
		
		match dir
		{
			Direction::Left => self.move_gap_horizontal(times, dir),
			Direction::Right => self.move_gap_horizontal(times, dir),
			
			Direction::Down => {
				let total = self.max_line_index();
				let target = (cur_pos.y + times).min(total);
				
				self.move_gap_vertical(target);
			},
			
			Direction::Up => {
				let times = times.min(cur_pos.y);
				self.move_gap_vertical(cur_pos.y - times);
			}
		}
	}
}

//matching
impl GapBuffer
{
	pub fn forward_match(&self, from: usize, matcher: u8) -> Option<usize>
	{
		let mut i = from;
		while i < self.logic_len()
		{
			if (self.char_at(i) == Some(matcher))
			{return Some(i);}
			
			i += 1;
		};
		
		return None;
	}
	
	pub fn backward_match(&self, from: usize, matcher: u8) -> Option<usize>
	{
		let mut i = from;
		loop
		{
			if (self.char_at(i) == Some(matcher))
			{return Some(i);}
			
			if (i == 0) {return None;}
			
			i -= 1;
		}
	}
}
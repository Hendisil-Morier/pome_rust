use crate::data_types::{Direction, Editor, Position};

impl Editor
{
  pub fn max_index_lines(&self) -> usize
  {
    self.buffer.len_lines().saturating_sub(1)
  }
  
  // didnt count newline
  pub fn line_len(&self, target_line: usize) -> usize
  {
    let target_line = self.max_index_lines().min(target_line);
    
    let line = self.buffer.line(target_line);
    let is_trailing_newline = line.chars().last() == Some('\n');

    let line_len = line.len_chars().saturating_sub
    (
      if is_trailing_newline {1}
      else {0}
    );
    
    line_len
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

//editing
impl Editor
{
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
	
	pub fn insert_at_cursor(&mut self, ch: char)
  {
    let cursor = &mut self.cur_info;
    
    self.buffer.insert_char(cursor.abs_pos, ch);
    cursor.abs_pos += 1;
  }
  
  pub fn delete_after(&mut self)
  {
    let cursor = &mut self.cur_info;
    
    self.buffer.remove(cursor.abs_pos .. cursor.abs_pos+1);
  }
  
  pub fn delete_before(&mut self)
  {
    let cursor = &mut self.cur_info;
    
    self.buffer.remove(cursor.abs_pos-1 .. cursor.abs_pos);
    cursor.abs_pos -= 1;
  }
}

//matching
impl Editor
{
  pub fn forward_match(&self, from_abs: usize, matcher: char)
  -> Option<usize>
  {
    let mut idx = from_abs;
    
    for ch in self.buffer.chars_at(from_abs)
    {
      if matcher == ch 
      {return Some(idx);}
      
      idx += 1;
    }
    
    None
  }
  
  pub fn backward_match(&self, from_abs: usize, matcher: char)
  -> Option<usize>
  {
    if from_abs >= self.buffer.len_chars()
    {return None;}
    
    let mut idx = from_abs + 1;
    let mut chars = self.buffer.chars_at(from_abs);
    
    while idx > 0
    {
      let ch = chars.prev()?;
      idx -= 1;
      
      if ch == matcher
      {return Some(idx);}
    }
    
    None
  }
}

//move cursor
impl Editor
{
  fn move_cursor_left(&mut self, times: usize)
  {
    let cursor = &mut self.cur_info;
    
    cursor.abs_pos = cursor.abs_pos.saturating_sub(times);
  }
  
  fn move_cursor_right(&mut self, times: usize)
  {
    let cursor = &mut self.cur_info;
    
    cursor.abs_pos = (cursor.abs_pos + times).min(self.buffer.len_chars());
  }
  
  fn move_cursor_up(&mut self, times: usize)
  {
    let cur_pos = self.cursor_pos();
    let target_y = cur_pos.y.saturating_sub(times);

    let target_abs_pos = self.repos_to_abspos(Position {x: cur_pos.x, y: target_y});
    
    self.cur_info.abs_pos = target_abs_pos;
  }
  
  fn move_cursor_down(&mut self, times: usize)
  {
    let cur_pos = self.cursor_pos();
    let target_y = (cur_pos.y + times).min(self.max_index_lines());
    
    let target_abs_pos = self.repos_to_abspos(Position { x: cur_pos.x, y: target_y });
    self.cur_info.abs_pos = target_abs_pos;
  }
  
  pub fn move_cursor(&mut self, times: usize, dir: Direction)
  {
    match dir
    {
      Direction::Left => self.move_cursor_left(times),
      Direction::Right => self.move_cursor_right(times),
      Direction::Up => self.move_cursor_up(times),
      Direction::Down => self.move_cursor_down(times),
    }
  }
  
  pub fn move_cursor_to(&mut self, abs_pos: usize)
  {
    let cursor = &mut self.cur_info;
    let abs_pos = abs_pos.min(self.buffer.len_chars());
    
    cursor.abs_pos = abs_pos;
  }
}

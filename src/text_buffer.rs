use crate::data_types::{Direction, Edit, Editor, Position};

//editing
impl Editor
{
	pub fn delete_selected(&mut self)
	{
		if self.cur_info.selecting == false
		{return;}
		
		let anchor;
		if let Some(a) = self.cur_info.anchor
		{anchor = a;}
		else {return;}
		
		let cur_abs_pos = self.cur_info.abs_pos;
		
		let start = anchor.min(cur_abs_pos);
		let end = anchor.max(cur_abs_pos);
		
		if start == end {return;}
		
		let removed = self.buffer.slice(start..end).to_string();
		self.buffer.remove(start..end);
		//update cursor pos
		self.cur_info.abs_pos = start;
		
		let record_edit = Edit::Delete { pos: start, text: removed};
		
		self.history.record(record_edit, cur_abs_pos, start);
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
	
	pub fn insert_char_at(&mut self, ch: char, pos: Position)
  {
    let cur_before = self.cur_info.abs_pos;
    let abs_pos = self.repos_to_abspos(pos);
    
    self.buffer.insert_char(abs_pos, ch);
    
    //if insert before cursor
    //then update pos
    if abs_pos < cur_before
    {self.cur_info.abs_pos += 1;}    
    
    let record_edit = Edit::Insert { pos: abs_pos, text: ch.to_string() };
    self.history.record(record_edit, cur_before, self.cur_info.abs_pos);
  }
  
  pub fn insert_string_at(&mut self, text: String, pos: Position)
  {
    let cur_before = self.cur_info.abs_pos;
    let abs_pos = self.repos_to_abspos(pos);
    
    self.buffer.insert(abs_pos, &text);
    
    //if insert before cursor
    //then update pos
    if abs_pos < cur_before
    {self.cur_info.abs_pos += text.chars().count();}    
    
    let record_edit = Edit::Insert { pos: abs_pos, text};
    self.history.record(record_edit, cur_before, self.cur_info.abs_pos);
  }
  
  pub fn delete_after(&mut self)
  {
    let cur_before = self.cur_info.abs_pos;
    if cur_before >= self.buffer.len_chars()
    {return;}
    
    //im fairly sure it wouldnt panic
    //well we 'bout to find out
    let removed = self.buffer.char(cur_before).to_string();
    
    self.buffer.remove(cur_before.. cur_before+1);
    
    let record_edit = Edit::Delete { pos: cur_before, text: removed};
    
    self.history.record(record_edit, cur_before, cur_before);
  }
  
  pub fn delete_before(&mut self)
  {
    let cursor = &mut self.cur_info;
    let cur_before = cursor.abs_pos;
    if cur_before == 0
    {return;}
    
    //it wont panic it wont panic 
    //it wont panic it wont panic 
    let removed = self.buffer.char(cur_before - 1).to_string();
    
    self.buffer.remove(cursor.abs_pos - 1 .. cursor.abs_pos);
    cursor.abs_pos = cursor.abs_pos - 1;
    
    let record_edit = Edit::Delete { pos: cur_before, text: removed};
    self.history.record(record_edit, cur_before, cur_before);
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

impl Editor
{
	pub fn quit(&mut self)
	{
		self.running = false;
	}
	
	pub fn update_scroll(&mut self, screen_h: usize)
	{
		let cur_pos = self.cursor_pos();
		let screen_rows = screen_h - 1;
		
		if cur_pos.y < self.row_offset 
		{self.row_offset = cur_pos.y;}
		
		if cur_pos.y >= self.row_offset + screen_rows 
		{self.row_offset = cur_pos.y - screen_rows + 1;}
	}
}
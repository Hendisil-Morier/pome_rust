
use crate::data_types::{editor::Editor, misc::Position};


impl Editor
{
	pub fn quit(&mut self)
	{
		self.running = false;
	}
	
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

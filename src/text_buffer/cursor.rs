use crate::data_types::{editor::Editor, misc::{Direction, Position}};

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

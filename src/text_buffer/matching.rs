use crate::data_types::editor::Editor;

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

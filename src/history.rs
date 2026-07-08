use std::time::{Duration, Instant};

use crate::data_types::{editor::Editor, history::{Edit, EditBatch, History}};

//helpers
impl History
{
  pub fn set_group_timeout(&mut self, timeout: Duration)
  {
    self.group_timeout = timeout;
  }
  
  /*get the last edit*/
  fn last_edit(&self)
  -> Option<&Edit>
  {
    self.current_batch.as_ref()?
      .edits.last()
  }
  
  pub fn can_undo(&self) -> bool
  {
    if self.position > 0
    { return true;}
    
    match &self.current_batch
    {
      Some(b) => !b.edits.is_empty(),
      None => false,
    }
  }
  
  pub fn can_redo(&self) -> bool
  {
    self.position < self.batches.len()
  }
  
  pub(crate) fn contiguous(last: &Edit, new: &Edit)
  -> bool
  {
    match (last, new)
    {
      //if two of 'em the same kind
      
      //and the next one start right after the last end
      (Edit::Insert { pos: last_p, text: last_t}, Edit::Insert { pos:next_p, ..})
      => *next_p == last_p + last_t.chars().count(),
      
      //or in deleting case, next_pos == last_pos
      //or next_pos == last_pos - 1
      //(cursor dont change or it move left)
      (Edit::Delete { pos: last_pos, .. }, Edit::Delete { pos: next_pos, ..})
      => *next_pos == *last_pos || next_pos + 1 == *last_pos,
      
      _ => false,
    }
  }
  
  pub(crate) fn commit_current_batch(&mut self)
  {
    let batch = match self.current_batch.take()
    {
      Some(b) => b,
      None => return,
    };
    
    if batch.edits.is_empty() {return;}
    
    self.batches.truncate(self.position);
    self.batches.push(batch);
    self.position = self.batches.len();
  }
}

impl History
{
  pub fn record(&mut self, edit: Edit, cursor_before: usize, cursor_after: usize)
  {
    let now = Instant::now();
    
    let mergable = match self.last_edit()
      {
        Some(last_edit) => self.explicit_group
        || (now.duration_since(self.last_edit_time) < self.group_timeout
        && Self::contiguous(last_edit, &edit)),

        None => false,
      };
      
    //close the last edit batch and start a new one
    if !mergable
    {
      self.commit_current_batch();
      self.current_batch = Some(EditBatch::new(cursor_before));
    }
    
    //current_batch should alway how Some(EditBatch)
    //at this point. unwrap panics means something fishy
    //going on
    let batch = self.current_batch.as_mut().unwrap();
    batch.edits.push(edit);
    batch.cursor_after = cursor_after;

    self.last_edit_time = now;
  }
}

impl History
{
  //return an undo batch
  //to operate on
  pub fn undo(&mut self)
  -> Option<&EditBatch>
  {
    //commit open batch
    self.commit_current_batch();
    
    if self.position == 0 {return None;}
    
    self.position -= 1;
    Some(&self.batches[self.position])
  }
  
  //return an redo batch
  //to operate on
  pub fn redo(&mut self)
  -> Option<&EditBatch>
  {
    self.commit_current_batch();
    
    if self.position >=self.batches.len()
    {return None;}
    
    let batch = &self.batches[self.position];
    
    //increment back from previous undo
    self.position += 1;
    return Some(batch);
  }
}

impl History
{
  pub fn begin_group(&mut self)
  -> bool
  {
    if self.explicit_group {return false;}
    
    self.commit_current_batch();
    self.explicit_group = true;
    
    true
  }
  
  pub fn end_group(&mut self)
  -> bool
  {
    if !self.explicit_group {return false;}
    
    self.explicit_group = false;
    self.commit_current_batch();
    
    true
  }
}

impl Editor
{
  pub fn undo(&mut self)
  {
    let batch = match self.history.undo()
      {
        Some(b) => b,
        None => return,
      };
    
    //walking the batch backward
    for edit in batch.edits.iter().rev()
    {
      match edit
      {
        Edit::Insert { pos, text } =>
        {
          let len = text.chars().count();
          self.buffer.remove(*pos .. *pos+len);
        }
        
        Edit::Delete { pos, text } =>
        {self.buffer.insert(*pos, text);}
      }
    }
    
    self.cur_info.abs_pos = batch.cursor_before.min(self.buffer.len_chars());
  }
  
  
  pub fn redo(&mut self)
  {
    let batch = match self.history.redo()
      {
        Some(b) => b,
        None => return,
      };
    
    //walking the batch forward
    for edit in batch.edits.iter()
    {
      match edit
      {
        Edit::Delete { pos, text } =>
        {
          let len = text.chars().count();
          self.buffer.remove(*pos .. *pos+len);
        }
        
        Edit::Insert { pos, text } =>
        {self.buffer.insert(*pos, text);}
      }
    }
    
    self.cur_info.abs_pos = batch.cursor_after.min(self.buffer.len_chars());
  }
}
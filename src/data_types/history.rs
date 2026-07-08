
#[derive(Clone, Debug)]
pub enum Edit
{
  Insert {pos: usize, text: String},
  Delete {pos: usize, text: String},
}

#[derive(Clone, Debug)]
pub struct EditBatch
{
  pub(crate) edits: Vec<Edit>,
  pub(crate) cursor_before: usize,
  pub(crate) cursor_after: usize,
}

impl EditBatch
{
  pub(crate) fn new(cursor_before: usize) -> Self
  {
    return Self
    {
      edits: Vec::new(),
      cursor_after: cursor_before,
      cursor_before,
    };
  }
}

pub struct History
{
  pub(crate) batches: Vec<EditBatch>,
  pub(crate) position: usize,
  //edit being accumulated, not yet committed to batches
  pub(crate) current_batch: Option<EditBatch>,
  
  pub(crate) last_edit_time: std::time::Instant,
  pub(crate) group_timeout: std::time::Duration,

  pub(crate) explicit_group: bool,
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
      group_timeout: std::time::Duration::from_millis(200),
      explicit_group: false,
    }
  }
}
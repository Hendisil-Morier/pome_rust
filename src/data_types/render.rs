use crate::data_types::misc::Position;



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
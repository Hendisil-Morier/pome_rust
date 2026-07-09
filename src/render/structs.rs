use ratatui::layout::Rect;
use ropey::Rope;

pub(crate) struct DrawContext <'a>
{
  pub(crate) rect: Rect,
  pub(crate) row_offset: usize,
  pub(crate) tab_width: usize,
  pub(crate) selection: Option<(usize, usize)>,
  pub(crate) rope: &'a Rope,
}

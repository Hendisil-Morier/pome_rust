use serde::Deserialize;

use crate::data_types::misc::{CursorInfo};

#[derive(Deserialize)]
#[serde(untagged)]
pub enum PanelColor
{
  Name(ratatui::style::Color),
  Rgb(u8, u8, u8),
}

type Rect = ratatui::layout::Rect;
#[derive(Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Panel
{
  Buffer
  {
    rect: Rect,
    row_offset: usize,
    #[serde(skip)]
    cursor: Option<CursorInfo>,
    #[serde(default = "default_tab_width")]
    tab_width: usize, //a bit weird
  },
  
  Text
  {
    rect: Rect,
    #[serde(default)]
    content: String,
    bg: Option<PanelColor>,
    fg: Option<PanelColor>,
  }
}

fn default_tab_width() -> usize {4}
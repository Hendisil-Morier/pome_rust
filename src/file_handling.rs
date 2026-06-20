use std::io::{BufReader, BufWriter};
use std::fs::File;
use ropey::Rope;

use crate::data_types::Editor;

pub fn load_file(editor: &mut Editor) -> std::io::Result<()>
{
	let path = match &editor.filename
		{
			Some(p) => p,
			None => return Ok(()),
		};
	
	let file = File::open(path)?;

	editor.buffer = Rope::from_reader(BufReader::new(file))?;
	
	return Ok(());
}

pub fn save_file(editor: &Editor) -> std::io::Result<()>
{
	let path = match &editor.filename
		{
			Some(p) => p,
			None => {
				let err = std::io::Error::new
				(
					std::io::ErrorKind::NotFound,
					"no filename, save aborted"
				);
				return Err(err);
			}
		};
	
	let file = File::create(path)?;
	
	editor.buffer.write_to(BufWriter::new(file))?;
	
	return Ok(());
}
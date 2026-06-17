use std::io::{Read, Write};
use std::fs::File;
use crate::editor::Editor;

pub fn load_file(editor: &mut Editor) -> std::io::Result<()>
{
	let path = match &editor.filename
		{
			Some(p) => p,
			None => return Ok(()),
		};
	
	let mut file = File::open(path)?;
	let mut content = Vec::new();
	file.read_to_end(&mut content)?;
	
	for byte in content
	{
		editor.buffer.insert(byte);
	}
	
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
	
	let mut file = File::create(path)?;
	let mut buf = Vec::<u8>::new();
	
	for i in 0..editor.buffer.logic_len()
	{
		if let Some(byte) = editor.buffer.char_at(i)
		{buf.push(byte);}
	}
	
	file.write_all(&buf)?;
	
	return Ok(());
}
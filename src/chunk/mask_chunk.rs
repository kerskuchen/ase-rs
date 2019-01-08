use std::io::{self, Read, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::helpers::{read_bytes, read_string};

pub struct MaskChunk {
	pub x_position: i16,
	pub y_position: i16,
	pub width: u16,
	pub height: u16,
	pub mask_name: String,
	pub bitmap_data: Vec<u8>,
}

impl MaskChunk {
	pub fn from_read<R>(read: &mut R) -> io::Result<Self>
	where
		R: Read + Seek,
	{
		let x_position = read.read_i16::<LittleEndian>()?;
		let y_position = read.read_i16::<LittleEndian>()?;
		let width = read.read_u16::<LittleEndian>()?;
		let height = read.read_u16::<LittleEndian>()?;
		read.seek(SeekFrom::Current(8))?;
		let mask_name = read_string(read)?;
		let bitmap_data = read_bytes(read, (height * (width + 7) / 8) as usize)?;

		Ok(Self {
			x_position,
			y_position,
			width,
			height,
			mask_name,
			bitmap_data,
		})
	}
}
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{self, Read, Seek};

use crate::Header;

mod cel_chunk;
pub use self::cel_chunk::*;
mod cel_extra_chunk;
pub use self::cel_extra_chunk::*;
mod color_profile_chunk;
pub use self::color_profile_chunk::*;
mod frame_tags_chunk;
pub use self::frame_tags_chunk::*;
mod layer_chunk;
pub use self::layer_chunk::*;
mod mask_chunk;
pub use self::mask_chunk::*;
mod old_palette_chunk4;
pub use self::old_palette_chunk4::*;
mod old_palette_chunk11;
pub use self::old_palette_chunk11::*;
mod palette_chunk;
pub use self::palette_chunk::*;
mod path_chunk;
pub use self::path_chunk::*;
mod slice_chunk;
pub use self::slice_chunk::*;
mod user_data_chunk;
pub use self::user_data_chunk::*;

pub enum ChunkData {
	CelChunk(CelChunk),
	CelExtraChunk(CelExtraChunk),
	ColorProfileChunk(ColorProfileChunk),
	FrameTagsChunk(FrameTagsChunk),
	LayerChunk(LayerChunk),
	MaskChunk(MaskChunk),
	OldPaletteChunk4(OldPaletteChunk4),
	OldPaletteChunk11(OldPaletteChunk11),
	PaletteChunk(PaletteChunk),
	PathChunk(PathChunk),
	SliceChunk(SliceChunk),
	UserDataChunk(UserDataChunk),
}

pub struct Chunk {
	pub chunk_size: u32,
	pub chunk_data: ChunkData,
}

impl Chunk {
	pub fn from_read<R>(read: &mut R, header: &Header) -> io::Result<Self>
	where
		R: Read + Seek,
	{
		let chunk_size = read.read_u32::<LittleEndian>()?;
		let chunk_type = read.read_u16::<LittleEndian>()?;
		let chunk_data = match chunk_type {
			0x0004 => ChunkData::OldPaletteChunk4(OldPaletteChunk4::from_read(read)?),
			0x0011 => ChunkData::OldPaletteChunk11(OldPaletteChunk11::from_read(read)?),
			0x2004 => ChunkData::LayerChunk(LayerChunk::from_read(read)?),
			0x2005 => ChunkData::CelChunk(CelChunk::from_read(read, chunk_size, header)?),
			0x2006 => ChunkData::CelExtraChunk(CelExtraChunk::from_read(read)?),
			0x2007 => ChunkData::ColorProfileChunk(ColorProfileChunk::from_read(read)?),
			0x2016 => ChunkData::MaskChunk(MaskChunk::from_read(read)?),
			0x2017 => ChunkData::PathChunk(PathChunk::from_read(read, chunk_size)?),
			_ => {
				return Err(io::Error::new(
					io::ErrorKind::Other,
					format!("Invalid Chunk Type {}", chunk_type),
				));
			}
		};

		let chunk = Chunk {
			chunk_size,
			chunk_data,
		};

		Ok(chunk)
	}
}
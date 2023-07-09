use anyhow::Result;

use crate::{sprite::Sprite, color::SpectrumColor};

// A cavern
#[derive(Debug)]
pub struct Cavern {
    pub layout: Layout,
    pub name: String,
    pub tile_sprites: Vec<Sprite>,
}

impl TryFrom<&[u8]> for Cavern {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8]) -> Result<Cavern> {
        anyhow::ensure!(bytes.len() == 1024, "Expected 1024 bytes");

        let layout = Layout::try_from(&bytes[0..512])?;
        let name = core::str::from_utf8(&bytes[512..544])?.to_owned();

        let mut tile_sprites = Vec::with_capacity(8);
        let mut pos = 544;
        for _ in 0..8 {
            let end = pos + 9;
            tile_sprites.push(Sprite::try_from_bytes(8, 8, &bytes[pos..end])?);
            pos = end;
        }


        Ok(Cavern { layout, name, tile_sprites })
    }
}


/// The layout of a cavern - a 32x16 grid of 8x8 pixel squares.
/// Each square is represented by a color attribute, and in turn
/// these color attributes index into background tile sprites for
/// the cavern.
#[derive(Debug)]
pub struct Layout {
    cells: Vec<SpectrumColor>
}

impl TryFrom<&[u8]> for Layout {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8]) -> Result<Layout> {
        anyhow::ensure!(bytes.len() == 512, "Expected 512 bytes");

        let mut cells: Vec<SpectrumColor> = Vec::with_capacity(512);

        for byte in bytes {
            cells.push(SpectrumColor::try_from(byte)?)
        }

        Ok(Layout { cells })
    }
}


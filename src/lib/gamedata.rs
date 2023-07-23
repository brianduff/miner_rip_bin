use std::{path::Path, fs::File, io::SeekFrom};

use anyhow::Result;
use crate::cavern::Cavern;
use std::io::{Read, Seek};

const CAVERNS_OFFSET: u64 = 0xb000;
const CAVERN_COUNT: usize = 20;
const CAVERN_DATA_SIZE_BYTES: usize = 1024;

#[derive(Debug)]
pub struct GameData {
  pub caverns: Vec<Cavern>
}

impl GameData {
  /// Load game data from a manic miner binary file.
  pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
    let mut file = File::open(path)?;
    let mut buf = vec![0; CAVERN_DATA_SIZE_BYTES];

    file.seek(SeekFrom::Start(CAVERNS_OFFSET))?;

    let mut caverns = Vec::with_capacity(CAVERN_COUNT);
    for _ in 0..CAVERN_COUNT {
        file.read_exact(&mut buf)?;
        let cavern = Cavern::try_from(&buf[..])?;
        caverns.push(cavern);
    }

    Ok(Self{ caverns })
  }

  pub fn cavern_tiles_rgba(&self) -> Result<Vec<u8>> {
    let sprites: Vec<_> = self.caverns.iter().flat_map(|c| &c.tile_sprites).collect();
    let columns = 16;

    // Fixme - hardcoded assumption that eash sprite is 8x8
    let width = 8 * columns;
    let rows = (sprites.len() / columns) + 1;
    let height  = 8 * rows;

    let mut merged = Vec::with_capacity(width * height * 4);

    for pixel_row in 0..height {
        for pixel_col in 0..width {
            let block_col = pixel_col / 8;
            let block_row = pixel_row / 8;
            let sprite_num = (block_row * columns) + block_col;

            let sprite_row = pixel_row % 8;
            let sprite_col = pixel_col % 8;

            if sprite_num < sprites.len() {
                //println!("[{},{},{}]", sprite_num, sprite_row, sprite_col);
                merged.push(sprites[sprite_num].to_rgba()[sprite_row][sprite_col*4]);
                merged.push(sprites[sprite_num].to_rgba()[sprite_row][sprite_col*4 + 1]);
                merged.push(sprites[sprite_num].to_rgba()[sprite_row][sprite_col*4 + 2]);
                merged.push(sprites[sprite_num].to_rgba()[sprite_row][sprite_col*4 + 3]);
            }
        }
    }

    let expected_count = width * height * 4;

    merged.resize(expected_count, 0);

    Ok(merged)

  }
}
use std::{fs::File, io::{Seek, SeekFrom, Read, BufWriter}, path::Path};

use color::SpectrumColor;
use anyhow::Result;

mod color;

fn main() -> Result<()> {
    let mut file = File::open("ManicMiner.bin")?;
    let mut buf = vec![0; 1024];
    file.seek(SeekFrom::Start(0xb000))?;

    let mut caverns = Vec::with_capacity(20);
    for _ in 0..20 {
        file.read_exact(&mut buf)?;
        let cavern = Cavern::try_from(&buf[..])?;
        caverns.push(cavern);
    }

    println!("Loaded {} caverns", caverns.len());

    let sprites: Vec<_> = caverns.iter().flat_map(|c| &c.tile_sprites).collect();
    export_sprites(sprites, "/tmp/newsprites.png")?;

    Ok(())
}

fn export_sprites<P: AsRef<Path>>(sprites: Vec<&Sprite>, path: P) -> Result<()> {
    let columns = 16;
    let file = File::create(path)?;
    let w = &mut BufWriter::new(file);

    println!("Got {} sprites with {} columns", sprites.len(), columns);

    // Fixme - hardcoded assumption that eash sprite is 8x8
    let width = 8 * columns;
    let rows = (sprites.len() / columns) + 1;
    let height  = 8 * rows;

    let mut encoder = png::Encoder::new(w, width as u32, height as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;

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

    writer.write_image_data(&merged)?;

    Ok(())

}

// A cavern
#[derive(Debug)]
struct Cavern {
    layout: Layout,
    name: String,
    tile_sprites: Vec<Sprite>,
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
struct Layout {
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

#[derive(Debug)]
struct Sprite {
    pixel_width: usize,
    pixel_height: usize,
    bytes: Vec<u8>,
    color: SpectrumColor
}

impl Sprite {
    fn try_from_bytes(pixel_width: usize, pixel_height: usize, bytes: &[u8]) -> Result<Self> {
        let bytes_per_row = pixel_width / 8;
        let rows = pixel_height;

        let expected_bytes = 1 + (rows * bytes_per_row); // +1 for attributes (color)
        anyhow::ensure!(bytes.len() == expected_bytes,
            "Expected {} bytes for a {}x{} sprite",
            expected_bytes, pixel_width, pixel_height);

        let color = SpectrumColor::try_from(&bytes[0])?;
        let bytes = bytes[1..].to_owned();

        Ok(Sprite{ pixel_width, pixel_height, bytes, color })
    }

    /// Converts this sprite into 2d rgba data.
    fn to_rgba(&self) -> Vec<Vec<u8>> {
        let mut result = Vec::with_capacity(self.pixel_height);
        let mut byte_index = 0;

        let mut byte_iter = self.bytes.iter();
        for pixel_row in 0..self.pixel_height {
            let mut cols: Vec<u8> = Vec::with_capacity(self.pixel_width);
            for b in 0..self.pixel_width / 8 {
                let byte = byte_iter.next().unwrap();
                let mut mask: u8 = 0b10000000;
                for _ in 0..8 {
                    cols.append(&mut if byte & mask != 0 {
                        self.color.ink_rgba()
                    } else {
                        self.color.paper_rgba()
                    });
                    mask >>= 1;
                }
            }
            result.push(cols);
        }

        result
    }
}
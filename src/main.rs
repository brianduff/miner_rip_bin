use std::{fs::File, io::{Seek, SeekFrom, Read, BufWriter}, path::Path};

use anyhow::Result;

use minerdata::sprite::Sprite;
use minerdata::cavern::Cavern;

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

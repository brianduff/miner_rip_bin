use std::{fs::File, io::BufWriter, path::Path};

use anyhow::Result;

use minerdata::gamedata::GameData;

fn main() -> Result<()> {
    let data = GameData::load("ManicMiner.bin")?;

    export_sprites(&data, "/tmp/newsprites.png")?;

    Ok(())
}

fn export_sprites<P: AsRef<Path>>(data: &GameData, path: P) -> Result<()> {
    let rgba = data.cavern_tiles_rgba()?;
    let sprite_count = data.caverns.iter().flat_map(|c| &c.tile_sprites).collect::<Vec<_>>().len();
    let columns = 16;
    let file = File::create(path)?;
    let w = &mut BufWriter::new(file);

    // Fixme - hardcoded assumption that eash sprite is 8x8
    let width = 8 * columns;
    let rows = (sprite_count / columns) + 1;
    let height  = 8 * rows;

    let mut encoder = png::Encoder::new(w, width as u32, height as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;


    writer.write_image_data(&rgba)?;

    Ok(())

}

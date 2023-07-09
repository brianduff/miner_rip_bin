use std::{path::Path, fs::File, io::SeekFrom};

use anyhow::Result;
use crate::cavern::Cavern;
use std::io::{Read, Seek};

const CAVERNS_OFFSET: u64 = 0xb000;
const CAVERN_COUNT: usize = 20;
const CAVERN_DATA_SIZE_BYTES: usize = 1024;

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
}
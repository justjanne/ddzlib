use crate::tile_header::read_tile_header;
use crate::tileset_meta::read_tileset;
use crate::{util, Error, TilesetMeta};
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;

#[derive(Debug)]
pub struct Tileset {
    pub meta: TilesetMeta,
    file: BufReader<File>,
}

impl Tileset {
    pub fn from_file<P: AsRef<Path>>(file: P) -> Result<Tileset, Error> {
        let file = File::open(file).map_err(|e| Error {
            message: format!("Could not open file: {0}", e),
        })?;
        let mut file = BufReader::new(file);
        let meta = util::read_buffer(read_tileset, &mut file)?;
        Ok(Tileset { meta, file })
    }

    pub fn read_tile(&mut self, zoom_level: usize, x: u32, y: u32) -> Result<Vec<u8>, Error> {
        let zoom_level = self.meta.zoom_levels.get(zoom_level).ok_or(Error {
            message: format!("zoom_level out of bounds: {0}", zoom_level),
        })?;
        let tile = zoom_level
            .tiles
            .get((y * zoom_level.columns + x) as usize)
            .ok_or(Error {
                message: format!(
                    "coordinates out of bound: {}:{} for zoom level with {}x{} tiles",
                    x, y, zoom_level.columns, zoom_level.rows
                ),
            })?;
        self.file
            .seek(SeekFrom::Start(tile.offset as u64))
            .map_err(|e| Error {
                message: format!("could not seek to tile: {0}", e),
            })?;
        let header = util::read_buffer(read_tile_header, &mut self.file)?;
        let mut buf = vec![0; header.length as usize];
        self.file.read_exact(&mut buf).map_err(|e| Error {
            message: format!("could not read tile: {0}", e),
        })?;
        Ok(buf)
    }
}

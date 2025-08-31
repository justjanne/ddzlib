use crate::file_type::{read_file_type, FileType};
use crate::zoom_level::read_zoom_level;
use crate::zoom_level_entry::read_zoom_level_entry;
use crate::ZoomLevel;
use nom::combinator::{flat_map, map};
use nom::multi::count;
use nom::number::streaming::le_u32;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Clone, Debug)]
pub struct TilesetMeta {
    pub file_type: FileType,
    pub version: u32,
    pub width: u32,
    pub height: u32,
    pub zoom_levels: Vec<ZoomLevel>,
}

pub(crate) fn read_tileset(input: &[u8]) -> IResult<&[u8], TilesetMeta> {
    map(
        tuple((
            read_file_type,    // file_type
            le_u32,            // version
            count(le_u32, 2),  // unknown
            le_u32,            // width
            le_u32,            // height
            count(le_u32, 18), // unknown
            flat_map(
                tuple((
                    read_zoom_level_entry,
                    read_zoom_level_entry,
                    read_zoom_level_entry,
                    read_zoom_level_entry,
                )),
                |(level0, level1, level2, level3)| {
                    map(
                        tuple((
                            read_zoom_level(level0),
                            read_zoom_level(level1),
                            read_zoom_level(level2),
                            read_zoom_level(level3),
                        )),
                        |(level0, level1, level2, level3)| vec![level0, level1, level2, level3],
                    )
                },
            ),
        )),
        move |(file_type, version, _, width, height, _, zoom_levels)| TilesetMeta {
            file_type,
            version,
            width,
            height,
            zoom_levels,
        },
    )(input)
}

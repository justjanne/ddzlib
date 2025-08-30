use crate::tile_entry::read_tile_entry;
use crate::{TileEntry, ZoomLevelEntry};
use nom::IResult;
use nom::combinator::map;
use nom::multi::count;

#[derive(Debug)]
pub struct ZoomLevel {
    pub columns: u32,
    pub rows: u32,
    pub tiles: Vec<TileEntry>,
}

pub(crate) fn read_zoom_level(
    entry: ZoomLevelEntry,
) -> impl FnMut(&[u8]) -> IResult<&[u8], ZoomLevel> {
    let columns = entry.columns;
    let rows = entry.rows;

    move |input| {
        map(
            count(read_tile_entry, (columns * rows) as usize),
            move |tiles| ZoomLevel {
                columns,
                rows,
                tiles,
            },
        )(input)
    }
}

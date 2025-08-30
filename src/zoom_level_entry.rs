use nom::combinator::map;
use nom::number::streaming::le_u32;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug)]
pub struct ZoomLevelEntry {
    pub start_index: u32,
    pub columns: u32,
    pub rows: u32,
}

pub(crate) fn read_zoom_level_entry(input: &[u8]) -> IResult<&[u8], ZoomLevelEntry> {
    map(
        tuple((le_u32, le_u32, le_u32)),
        move |(start_index, columns, rows)| ZoomLevelEntry {
            start_index,
            columns,
            rows,
        },
    )(input)
}

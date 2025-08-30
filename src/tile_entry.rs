use nom::combinator::map;
use nom::number::streaming::le_u32;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug)]
pub struct TileEntry {
    pub offset: u32,
    pub length: u32,
}

pub(crate) fn read_tile_entry(input: &[u8]) -> IResult<&[u8], TileEntry> {
    map(
        tuple((le_u32, le_u32)),
        move |(offset, length)| TileEntry {
            offset,
            length,
        },
    )(input)
}

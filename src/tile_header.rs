use nom::bytes::streaming::tag;
use nom::combinator::map;
use nom::number::streaming::le_u32;
use nom::sequence::tuple;
use nom::IResult;
use nom::number::complete::le_u16;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TileHeader {
    pub format: u16,
    pub length: u32,
}

pub(crate) fn read_tile_header(input: &[u8]) -> IResult<&[u8], TileHeader> {
    map(
        tuple((
            tag("WP"),
            le_u16,
            le_u32,
        )),
        move |(_, format, length)| TileHeader {
            format,
            length,
        },
    )(input)
}

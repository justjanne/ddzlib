use nom::bytes::streaming::tag;
use nom::combinator::map;
use nom::number::streaming::le_u32;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TileHeader {
    pub length: u32,
}

pub(crate) fn read_tile_header(input: &[u8]) -> IResult<&[u8], TileHeader> {
    map(
        tuple((
            tag("WP\0\x18"),
            le_u32,
        )),
        move |(_, length)| TileHeader {
            length,
        },
    )(input)
}

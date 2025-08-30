use log::warn;
use nom::combinator::map_res;
use nom::IResult;
use nom::number::streaming::be_u32;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum FileType {
    WEBP,
}

pub(crate) fn read_file_type(input: &[u8]) -> IResult<&[u8], FileType> {
    map_res(be_u32, |data| {
        warn!("read file type: {:?}", data);
        match data {
            0x57454250 => Ok(FileType::WEBP),
            _ => Err(format!("Unable to parse file type: {data}")),
        }
    })(input)
}
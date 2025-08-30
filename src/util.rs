use crate::Error;
use nom::{IResult, Offset};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub(crate) fn read_file<F, P: AsRef<Path>, O>(parser: &mut F, file: P) -> Result<O, Error>
where
    F: FnMut(&[u8]) -> IResult<&[u8], O>,
{
    let file = File::open(file).map_err(|e| Error {
        message: format!("Could not open file: {0}", e),
    })?;
    let mut buf = BufReader::new(file);
    loop {
        let opt = match parser(buf.buffer()) {
            Ok((data, parsed)) => Some((buf.buffer().offset(data), parsed)),
            Err(nom::Err::Incomplete(_)) => None,
            Err(nom::Err::Error(e)) => {
                return Err(Error {
                    message: format!("could not parse tileset: {:#?}", e),
                });
            }
            Err(nom::Err::Failure(e)) => {
                return Err(Error {
                    message: format!("could not parse tileset: {:#?}", e),
                });
            }
        };

        match opt {
            Some((sz, o)) => {
                buf.consume(sz);
                return Ok(o);
            }
            None => {
                buf.fill_buf().map_err(|e| Error {
                    message: format!("could not read tileset: {:#?}", e),
                })?;
            }
        }
    }
}

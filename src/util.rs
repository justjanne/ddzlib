use std::io::{BufRead, BufReader};
use std::fs::File;
use nom::{IResult, Offset};
use crate::Error;

pub fn read_buffer<F, T>(mut parser: F, file: &mut BufReader<File>) -> Result<T, Error>
where
    F: FnMut(&[u8]) -> IResult<&[u8], T>,
{
    loop {
        let opt = match parser(file.buffer()) {
            Ok((data, parsed)) => Some((file.buffer().offset(data), parsed)),
            Err(nom::Err::Incomplete(_)) => None,
            Err(nom::Err::Error(e)) => {
                break Err(Error {
                    message: format!("could not parse buffer: {:#?}", e),
                });
            }
            Err(nom::Err::Failure(e)) => {
                break Err(Error {
                    message: format!("could not parse buffer: {:#?}", e),
                });
            }
        };

        match opt {
            Some((sz, o)) => {
                file.consume(sz);
                break Ok(o);
            }
            None => {
                file.fill_buf().map_err(|e| Error {
                    message: format!("could not read buffer: {:#?}", e),
                })?;
            }
        }
    }
}

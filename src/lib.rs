pub mod schema;
use std::{io::BufRead, string::FromUtf8Error};

use quick_xml::{Reader, events::Event, de::from_str};
use schema::Page;


pub struct Undumper<R: BufRead> {
    reader: Reader<R>,
    buffer: Vec<u8>,
    started: bool
}

#[derive(Debug)]
pub enum Error {
    QuickXmlError(quick_xml::Error),
    QuickXmlDesearializeError(quick_xml::de::DeError),
    BufferDecodeError(FromUtf8Error)
}

impl <R:BufRead> Undumper<R> {
    pub fn from_reader(reader: R) -> Self {
        Self { reader:Reader::from_reader(reader), buffer: Vec::new(), started: false }
    }
}



impl<R: BufRead> Iterator for Undumper<R> {
    type Item = Result<Page, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            // Reads until first page element
            match parse_to_start(self) {
                Err(e) => return Some(Err(e)),
                Ok(Some(())) => {
                    self.started = true;
                },
                Ok(None) => {
                    return None
                }
            }
        }

        self.buffer.clear();

        // Reads into the buffer until the page element close
        let result = self.reader.read_to_end_into(quick_xml::name::QName(b"page"), &mut self.buffer);
        match result {
            Err(e) => return Some(Err(Error::QuickXmlError(e))),
            _ => {}
        }

        let string_buffer: String;
        match String::from_utf8(self.buffer.clone()) {
            Err(e) => return Some(Err(Error::BufferDecodeError(e))),
            Ok(s) => string_buffer = s
        };

        let page_res = from_str::<Page>(&string_buffer);
        match page_res {
            Err(e) => return Some(Err(Error::QuickXmlDesearializeError(e))),
            Ok(p) => return Some(Ok(p))
        }
    }
}

/// Returns Some(()) if it read to page
/// Returns None if the Eof was reached
fn parse_to_start<R: BufRead>(undumper: &mut Undumper<R>) -> Result<Option<()>, Error> {
    loop {
        match undumper.reader.read_event_into(&mut undumper.buffer) {
            Err(e) => {
                return Err(Error::QuickXmlError(e))
            },
            Ok(e) => {
                match e {
                    Event::Start(e) => {
                        if e.name().as_ref() == b"page" {
                            break
                        }
                    },
                    Event::Eof => return Ok(None),
                    _ => {}
                }
            }
        }
    }
    Ok(Some(()))
}


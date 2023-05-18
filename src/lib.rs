pub mod schema;
use std::{io::BufRead, string::FromUtf8Error};

use quick_xml::{Reader, events::{Event, BytesStart}, de::from_str, Writer};
use schema::Page;


pub struct Undumper<R: BufRead> {
    xml_reader: Reader<R>,
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
        Self { xml_reader:Reader::from_reader(reader), buffer: Vec::new(), started: false }
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
            self.buffer.clear();
        }


        // Reads into the buffer until the page element close
        let result = read_to_end_into_buffer(&mut self.xml_reader, &BytesStart::new("page"), &mut self.buffer);
        self.started=false;

        match result {
            Err(e) => return Some(Err(Error::QuickXmlError(e))),
            Ok(output_buf) => {
                let output_string = String::from_utf8_lossy(&output_buf);

                //println!("PAGE: '{}'", output_string.clone());

                let page_res = from_str::<Page>(&output_string);
                match page_res {
                    Err(e) => return Some(Err(Error::QuickXmlDesearializeError(e))),
                    Ok(p) => return Some(Ok(p))
                }
            }
        }
    }
}

/// Returns Some(()) if it read to page
/// Returns None if the Eof was reached
fn parse_to_start<R: BufRead>(undumper: &mut Undumper<R>) -> Result<Option<()>, Error> {
    //println!("PARSING TO START");
    loop {
        match undumper.xml_reader.read_event_into(&mut undumper.buffer) {
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


/// https://github.com/capnfabs/trackscan/blob/main/src/main.rs
fn read_to_end_into_buffer<R: BufRead>(
    reader: &mut Reader<R>,
    start_tag: &BytesStart,
    junk_buf: &mut Vec<u8>,
) -> Result<Vec<u8>, quick_xml::Error> {
    let mut depth = 0;
    let mut output_buf: Vec<u8> = Vec::new();
    let mut w = Writer::new(&mut output_buf);
    let tag_name = start_tag.name();
    w.write_event(Event::Start(start_tag.clone()))?;
    let mut i = 0;
    loop {
        junk_buf.clear();
        let event = reader.read_event_into(junk_buf)?;
        w.write_event(&event)?;

        //println!("GOT EVENT {} {:?}", i, event);
        i+=1;

        match event {
            Event::Start(e) if e.name() == tag_name => depth += 1,
            Event::End(e) if e.name() == tag_name => {
                if depth == 0 {
                    return Ok(output_buf);
                }
                depth -= 1;
            }
            Event::Eof => {
                panic!("oh no")
            }
            _ => {}
        }
    }
}




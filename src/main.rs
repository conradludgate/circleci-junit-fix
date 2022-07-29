// use memchr::memchr;
use quick_xml::events::attributes::Attribute;
use quick_xml::events::{BytesStart, BytesText, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::borrow::Cow;
use std::io::{stdin, stdout, BufRead, BufReader, Stdin, Write};

struct Context<R: BufRead> {
    reader: Reader<R>,
    failure: Vec<u8>,
}

impl Context<BufReader<Stdin>> {
    pub fn new() -> Self {
        let mut reader = Reader::from_reader(BufReader::new(stdin()));
        reader.trim_text(true);
        Self {
            reader,
            failure: Vec::new(),
        }
    }
}

impl<R: BufRead> Context<R> {
    pub fn read_event<'b>(&mut self, buf: &'b mut Vec<u8>) -> quick_xml::Result<Event<'b>> {
        buf.clear();
        self.reader.read_event(buf)
    }
}

fn main() -> quick_xml::Result<()> {
    let mut writer = Writer::new(stdout());
    let mut ctx = Context::new();
    let mut buf = Vec::with_capacity(1024);

    loop {
        match ctx.read_event(&mut buf)? {
            Event::Start(s) if s.name() == b"testsuites" => {
                writer.write_event(Event::Start(s))?;
                ctx.testsuites(&mut writer, &mut buf)?
            }
            Event::Eof => break Ok(()),
            e => writer.write_event(e)?,
        }
    }
}

impl<R: BufRead> Context<R> {
    fn testsuites<'b, W: Write>(
        &mut self,
        writer: &mut Writer<W>,
        buf: &'b mut Vec<u8>,
    ) -> quick_xml::Result<()> {
        loop {
            match self.read_event(buf)? {
                Event::Start(s) if s.name() == b"testsuite" => {
                    writer.write_event(Event::Start(s))?;
                    self.testsuite(writer, buf)?
                }
                Event::End(e) if e.name() == b"testsuites" => {
                    writer.write_event(Event::End(e))?;
                    break Ok(());
                }
                e => {
                    break Err(quick_xml::Error::UnexpectedEof(format!(
                        "expected to parse testsuite or end testsuites, got {e:?}"
                    )))
                }
            }
        }
    }

    fn testsuite<'b, W: Write>(
        &mut self,
        writer: &mut Writer<W>,
        buf: &'b mut Vec<u8>,
    ) -> quick_xml::Result<()> {
        loop {
            match self.read_event(buf)? {
                Event::Start(s) if s.name() == b"testcase" => {
                    writer.write_event(Event::Start(s))?;
                    self.testcase(writer, buf)?
                }
                Event::End(e) if e.name() == b"testsuite" => {
                    writer.write_event(Event::End(e))?;
                    break Ok(());
                }
                e => {
                    break Err(quick_xml::Error::UnexpectedEof(format!(
                        "expected to parse testcase or end testsuite, got {e:?}"
                    )))
                }
            }
        }
    }

    fn testcase<'b, W: Write>(
        &mut self,
        writer: &mut Writer<W>,
        buf: &'b mut Vec<u8>,
    ) -> quick_xml::Result<()> {
        loop {
            match self.read_event(buf)? {
                Event::Start(s) if s.name() == b"failure" => {
                    self.failure(writer, buf)?;
                }
                e @ Event::Text(_) => {
                    writer.write_event(e)?;
                }
                Event::Start(s) if s.name() == b"system-out" => {
                    writer.write_event(Event::Start(s))?;
                }
                Event::End(s) if s.name() == b"system-out" => {
                    writer.write_event(Event::End(s))?;
                }
                Event::Start(s) if s.name() == b"system-err" => {
                    writer.write_event(Event::Start(s))?;
                }
                Event::End(s) if s.name() == b"system-err" => {
                    writer.write_event(Event::End(s))?;
                }
                Event::End(e) if e.name() == b"testcase" => {
                    writer.write_event(Event::End(e))?;
                    break Ok(());
                }
                e => {
                    break Err(quick_xml::Error::UnexpectedEof(format!(
                        "expected to parse testcase or end testsuite, got {e:?}"
                    )))
                }
            }
        }
    }

    fn failure<'b, W: Write>(
        &mut self,
        writer: &mut Writer<W>,
        buf: &'b mut Vec<u8>,
    ) -> quick_xml::Result<()> {
        self.failure.clear();

        loop {
            match self.read_event(buf)? {
                Event::Text(s) => {
                    self.failure.extend_from_slice(s.escaped());
                }
                Event::End(s) if s.name() == b"failure" => {
                    let mut start = BytesStart::borrowed_name(b"failure");
                    start.push_attribute(Attribute {
                        key: b"type",
                        value: Cow::Borrowed(b"test failure"),
                    });
                    start.push_attribute(Attribute {
                        key: b"message",
                        value: Cow::Borrowed(&self.failure),
                    });
                    writer.write_event(Event::Start(start))?;
                    writer.write_event(Event::Text(BytesText::from_escaped(&self.failure)))?;
                    writer.write_event(Event::End(s))?;
                    break Ok(());
                }
                e => {
                    break Err(quick_xml::Error::UnexpectedEof(format!(
                        "expected to parse testcase or end testsuite, got {e:?}"
                    )))
                }
            }
        }
    }
}

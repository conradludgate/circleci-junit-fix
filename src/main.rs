use quick_xml::events::attributes::Attribute;
use quick_xml::events::{BytesStart, BytesText, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::borrow::Cow;
use std::io::{stdin, stdout, BufRead, BufReader, Stdin, Write};

struct Context<'w, R: BufRead, W: Write> {
    reader: Reader<R>,
    writer: &'w mut Writer<W>,
}

impl<'w, W: Write> Context<'w, BufReader<Stdin>, W> {
    pub fn new(writer: &'w mut Writer<W>) -> Self {
        let mut reader = Reader::from_reader(BufReader::new(stdin()));
        reader.trim_text(true);
        Self { reader, writer }
    }
}

impl<'w, R: BufRead, W: Write> Context<'w, R, W> {
    pub fn read_event<'b>(&mut self, buf: &'b mut Vec<u8>) -> quick_xml::Result<Event<'b>> {
        buf.clear();
        self.reader.read_event(buf)
    }

    pub fn write_event(&mut self, e: Event<'_>) -> quick_xml::Result<()> {
        self.writer.write_event(e)
    }
}

fn main() -> quick_xml::Result<()> {
    let mut writer = Writer::new(stdout());
    let mut ctx = Context::new(&mut writer);
    let mut buf = Vec::with_capacity(1024);

    loop {
        match ctx.read_event(&mut buf)? {
            Event::Start(s) if s.name() == b"testsuites" => {
                ctx.write_event(Event::Start(s))?;
                ctx.testsuites(&mut buf)?
            }
            Event::Eof => break Ok(()),
            e => ctx.write_event(e)?,
        }
    }
}

impl<R: BufRead, W: Write> Context<'_, R, W> {
    fn testsuites(&mut self, buf: &mut Vec<u8>) -> quick_xml::Result<()> {
        loop {
            match self.read_event(buf)? {
                Event::Start(s) if s.name() == b"testsuite" => {
                    self.write_event(Event::Start(s))?;
                    self.testsuite(buf)?
                }
                Event::End(e) if e.name() == b"testsuites" => {
                    self.write_event(Event::End(e))?;
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

    fn testsuite(&mut self, buf: &mut Vec<u8>) -> quick_xml::Result<()> {
        loop {
            match self.read_event(buf)? {
                Event::Start(s) if s.name() == b"testcase" => {
                    self.write_event(Event::Start(s))?;
                    self.testcase(buf)?
                }
                Event::End(e) if e.name() == b"testsuite" => {
                    self.write_event(Event::End(e))?;
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

    fn testcase(&mut self, buf: &mut Vec<u8>) -> quick_xml::Result<()> {
        loop {
            match self.read_event(buf)? {
                Event::Start(s) if s.name() == b"failure" => {
                    self.failure(buf)?;
                }
                e @ Event::Text(_) => {
                    self.write_event(e)?;
                }
                Event::Start(s)
                    if s.name() == b"system-out"
                        || s.name() == b"system-err"
                        || s.name() == b"rerunFailure" =>
                {
                    self.write_event(Event::Start(s))?;
                }
                Event::End(s)
                    if s.name() == b"system-out"
                        || s.name() == b"system-err"
                        || s.name() == b"rerunFailure" =>
                {
                    self.write_event(Event::End(s))?;
                }
                Event::End(e) if e.name() == b"testcase" => {
                    self.write_event(Event::End(e))?;
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

    fn failure(&mut self, buf: &mut Vec<u8>) -> quick_xml::Result<()> {
        let mut failure = Vec::new();

        loop {
            match self.read_event(buf)? {
                Event::Text(s) => {
                    failure = s.escaped().to_owned();
                }
                Event::End(s) if s.name() == b"failure" => {
                    let mut start = BytesStart::borrowed_name(b"failure");
                    start.push_attribute(Attribute {
                        key: b"type",
                        value: Cow::Borrowed(b"test failure"),
                    });
                    start.push_attribute(Attribute {
                        key: b"message",
                        value: Cow::Borrowed(&failure),
                    });
                    self.write_event(Event::Start(start))?;
                    self.write_event(Event::Text(BytesText::from_plain(&failure)))?;
                    self.write_event(Event::End(s))?;
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

use quick_xml::{
    events::{attributes::Attribute, Event},
    Reader, Writer,
};
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
                e => self.write_event(e)?,
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
                e => self.write_event(e)?,
            }
        }
    }

    fn testcase(&mut self, buf: &mut Vec<u8>) -> quick_xml::Result<()> {
        loop {
            match self.read_event(buf)? {
                Event::Start(mut s) if s.name() == b"failure" => {
                    s.push_attribute(Attribute {
                        key: b"message",
                        value: Cow::Borrowed(b""),
                    });
                    self.write_event(Event::Start(s))?;
                }
                Event::End(e) if e.name() == b"testcase" => {
                    self.write_event(Event::End(e))?;
                    break Ok(());
                }
                e => self.write_event(e)?,
            }
        }
    }
}

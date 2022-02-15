use memchr::memchr;
use quick_xml::events::attributes::Attribute;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::borrow::Cow;
use std::io::{stdin, stdout, BufReader};

fn main() -> quick_xml::Result<()> {
    let mut reader = Reader::from_reader(BufReader::new(stdin()));
    reader.trim_text(true);
    let mut writer = Writer::new(stdout());
    let mut buf = Vec::new();

    let mut failure = false;
    let mut inside: Option<bool> = None; // true is stdout, false is stderr
    let mut stdout = b"--- STDOUT:\n".to_vec();
    let mut stderr = b"--- STDERR:&#xA;".to_vec();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) if e.name() == b"system-out" => {
                inside = Some(true);
            }
            Ok(Event::Start(ref e)) if e.name() == b"system-err" => {
                inside = Some(false);
            }
            Ok(Event::End(ref e)) if e.name() == b"system-out" => {
                inside = None;
            }
            Ok(Event::End(ref e)) if e.name() == b"system-err" => {
                inside = None;
            }
            Ok(Event::Text(ref e)) => match inside {
                Some(true) => stdout.extend_from_slice(e),
                Some(false) => replace_nl(e, &mut stderr),
                None => {}
            },
            Ok(Event::Empty(ref e)) if e.name() == b"failure" => {
                failure = true;
            }
            Ok(Event::End(ref e)) if e.name() == b"testcase" => {
                if failure {
                    let mut failure = BytesStart::owned(b"failure".to_vec(), "failure".len());
                    failure.push_attribute(Attribute {
                        key: b"message",
                        value: Cow::Borrowed(&stderr),
                    });
                    writer.write_event(Event::Start(failure))?;
                    writer.write_event(Event::Text(BytesText::from_plain(&stdout)))?;

                    stdout.clear();
                    stdout.extend_from_slice(b"--- STDOUT:\n");
                    stderr.clear();
                    stderr.extend_from_slice(b"--- STDERR:&#xA;");

                    writer.write_event(Event::End(BytesEnd::borrowed(b"failure")))?;
                }
                failure = false;

                writer.write_event(Event::End(BytesEnd::borrowed(b"testcase")))?;
            }
            Ok(Event::Eof) => break,
            Ok(e) => assert!(writer.write_event(e).is_ok()),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
        }
        buf.clear();
    }

    Ok(())
}

fn replace_nl(mut bytes: &[u8], out: &mut Vec<u8>) {
    out.reserve(bytes.len());

    while let Some(idx) = memchr(b'\n', bytes) {
        let (start, rest) = bytes.split_at(idx);
        bytes = rest.split_at(1).1;

        out.extend_from_slice(start);
        out.extend_from_slice(b"&#xA;");
    }

    out.extend_from_slice(bytes);
}

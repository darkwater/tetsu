use std::io;

pub struct LogProxy;

impl io::Write for LogProxy {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if let Some(ref mut progress_bar) = *crate::PROGRESS_BAR.write().unwrap() {
            let s = String::from_utf8_lossy(buf).to_string();
            for line in s.lines() {
                progress_bar.println(line)?;
            }

            Ok(buf.len())
        } else {
            let mut stdout = io::stdout();
            stdout.write_all(buf)?;
            stdout.flush()?;
            Ok(buf.len())
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        io::stdout().flush()
    }
}

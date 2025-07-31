use std::io::{BufRead, BufReader, Read};
use std::os::fd::{AsRawFd, RawFd};

pub struct NonblockingBufReader {
    buffered: BufReader<RawFd2>,
}

struct RawFd2 {
    fd: RawFd,
}

impl NonblockingBufReader {
    /// Takes ownership of the underlying FD
    pub fn new<R: AsRawFd>(underlying: R) -> NonblockingBufReader {
        let buffered = BufReader::new(RawFd2 {
            fd: underlying.as_raw_fd(),
        });
        return NonblockingBufReader { buffered };
    }

    /// Does BufReader::read_line but only if there's already at
    /// least one byte available on the FD. In case of EOF, returns
    /// an empty string.
    /// Possible outcomes: (0) no-data-yet, (1) data, (2) EOF, (3) Error
    pub fn read_char_only_if_data(&mut self) -> std::io::Result<Option<u8>> {
        let r = unsafe {
            // The reason this is safe is we know 'inner' wraps a valid FD,
            // and we're not doing any reads on it such as would disturb BufReader.
            let fd = self.buffered.get_ref().fd;
            let flags = libc::fcntl(fd, libc::F_GETFL);
            libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK);
            let r = self.buffered.fill_buf();
            libc::fcntl(fd, libc::F_SETFL, flags);
            r
        };
        // Behavior of fill_buf is "Returns the contents of the internal buffer,
        // filling it with more data from the inner reader if it is empty."
        // If there were no bytes available, then (1) the internal buffer is
        // empty, (2) it'll call inner.read(), (3) that call will error WouldBlock.
        match r {
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok(None), // (0) no-data-yet
            Ok(buf) if !buf.is_empty() => {
                let mut line: [u8; 1] = [0];
                self.buffered.read_exact(&mut line).unwrap();
                Ok(Some(line[0])) // (1) data, or further error
            }
            Ok(_) => Ok(Some(0)), // (2) EOF
            Err(e) => Err(e),     // (3) Error
        }
    }
}

// Here's a private implementation of 'Read' for raw file descriptors,
// for use in BufReader...
impl std::io::Read for RawFd2 {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        assert!(buf.len() <= isize::max_value() as usize);
        match unsafe { libc::read(self.fd, buf.as_mut_ptr() as _, buf.len()) } {
            x if x < 0 => Err(std::io::Error::last_os_error()),
            x => Ok(x as usize),
        }
    }
}

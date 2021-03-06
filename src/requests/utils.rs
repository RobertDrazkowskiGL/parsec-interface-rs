// Copyright 2019 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0
//! Module containing helper macros and functions used in tests.
#![macro_use]
macro_rules! get_from_stream {
    ($stream:expr, $type:ty) => {
        match &mut $stream {
            stream => {
                let mut read_bytes = [0_u8; std::mem::size_of::<$type>()];
                stream.read_exact(&mut read_bytes)?;
                <$type>::from_le_bytes(read_bytes)
            }
        }
    };
    ($stream:expr; $size:expr) => {
        match (&mut $stream, $size) {
            (stream, size) => {
                let mut read_bytes = vec![0; size];
                stream.read_exact(&mut read_bytes)?;
                read_bytes
            }
        }
    };
}

#[cfg(test)]
pub mod tests {
    use std::io::{Error, ErrorKind, Read, Result, Write};

    #[derive(Debug)]
    pub struct MockReadWrite {
        pub buffer: Vec<u8>,
    }

    impl Read for MockReadWrite {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            for val in buf.iter_mut() {
                *val = self.buffer.remove(0);
            }

            Ok(buf.len())
        }
    }

    impl Write for MockReadWrite {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            for val in buf.iter() {
                self.buffer.push(*val);
            }
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub struct MockFailReadWrite;

    impl Read for MockFailReadWrite {
        fn read(&mut self, _: &mut [u8]) -> Result<usize> {
            Err(Error::from(ErrorKind::Other))
        }
    }

    impl Write for MockFailReadWrite {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Err(Error::from(ErrorKind::Other))
        }

        fn flush(&mut self) -> Result<()> {
            Err(Error::from(ErrorKind::Other))
        }
    }
}

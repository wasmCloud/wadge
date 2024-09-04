use crate::bindings::{exports, wasi};
use crate::Handler;

use exports::wasi::io::poll::PollableBorrow;
use exports::wasi::io::streams::InputStreamBorrow;

use wasi::io::error::Error;
use wasi::io::poll::{self, Pollable};
use wasi::io::streams::{InputStream, OutputStream, StreamError};

impl From<StreamError> for exports::wasi::io::streams::StreamError {
    fn from(value: StreamError) -> Self {
        match value {
            StreamError::LastOperationFailed(err) => {
                Self::LastOperationFailed(exports::wasi::io::error::Error::new(err))
            }
            StreamError::Closed => Self::Closed,
        }
    }
}

impl exports::wasi::io::error::Guest for Handler {
    type Error = Error;
}

impl exports::wasi::io::error::GuestError for Error {
    fn to_debug_string(&self) -> String {
        Self::to_debug_string(self)
    }
}

impl exports::wasi::io::streams::Guest for Handler {
    type InputStream = InputStream;
    type OutputStream = OutputStream;
}

impl exports::wasi::io::streams::GuestInputStream for InputStream {
    fn read(&self, len: u64) -> Result<Vec<u8>, exports::wasi::io::streams::StreamError> {
        let ret = Self::read(self, len)?;
        Ok(ret)
    }

    fn blocking_read(&self, len: u64) -> Result<Vec<u8>, exports::wasi::io::streams::StreamError> {
        let ret = Self::blocking_read(self, len)?;
        Ok(ret)
    }

    fn skip(&self, len: u64) -> Result<u64, exports::wasi::io::streams::StreamError> {
        let ret = Self::skip(self, len)?;
        Ok(ret)
    }

    fn blocking_skip(&self, len: u64) -> Result<u64, exports::wasi::io::streams::StreamError> {
        let ret = Self::blocking_skip(self, len)?;
        Ok(ret)
    }

    fn subscribe(&self) -> exports::wasi::io::poll::Pollable {
        exports::wasi::io::poll::Pollable::new(Self::subscribe(self))
    }
}

impl exports::wasi::io::streams::GuestOutputStream for OutputStream {
    fn check_write(&self) -> Result<u64, exports::wasi::io::streams::StreamError> {
        let ret = Self::check_write(self)?;
        Ok(ret)
    }

    fn write(&self, contents: Vec<u8>) -> Result<(), exports::wasi::io::streams::StreamError> {
        Self::write(self, &contents)?;
        Ok(())
    }

    fn blocking_write_and_flush(
        &self,
        contents: Vec<u8>,
    ) -> Result<(), exports::wasi::io::streams::StreamError> {
        Self::blocking_write_and_flush(self, &contents)?;
        Ok(())
    }

    fn flush(&self) -> Result<(), exports::wasi::io::streams::StreamError> {
        Self::flush(self)?;
        Ok(())
    }

    fn blocking_flush(&self) -> Result<(), exports::wasi::io::streams::StreamError> {
        Self::blocking_flush(self)?;
        Ok(())
    }

    fn subscribe(&self) -> exports::wasi::io::poll::Pollable {
        exports::wasi::io::poll::Pollable::new(Self::subscribe(self))
    }

    fn write_zeroes(&self, len: u64) -> Result<(), exports::wasi::io::streams::StreamError> {
        Self::write_zeroes(self, len)?;
        Ok(())
    }

    fn blocking_write_zeroes_and_flush(
        &self,
        len: u64,
    ) -> Result<(), exports::wasi::io::streams::StreamError> {
        Self::blocking_write_zeroes_and_flush(self, len)?;
        Ok(())
    }

    fn splice(
        &self,
        src: InputStreamBorrow<'_>,
        len: u64,
    ) -> Result<u64, exports::wasi::io::streams::StreamError> {
        let ret = Self::splice(self, src.get(), len)?;
        Ok(ret)
    }

    fn blocking_splice(
        &self,
        src: InputStreamBorrow<'_>,
        len: u64,
    ) -> Result<u64, exports::wasi::io::streams::StreamError> {
        let ret = Self::blocking_splice(self, src.get(), len)?;
        Ok(ret)
    }
}

impl exports::wasi::io::poll::Guest for Handler {
    type Pollable = Pollable;

    fn poll(in_: Vec<PollableBorrow<'_>>) -> Vec<u32> {
        poll::poll(
            &in_.iter()
                .map(exports::wasi::io::poll::PollableBorrow::get)
                .collect::<Vec<_>>(),
        )
    }
}

impl exports::wasi::io::poll::GuestPollable for Pollable {
    fn ready(&self) -> bool {
        Self::ready(self)
    }

    fn block(&self) {
        Self::block(self);
    }
}

mod bindings {
    use crate::Handler;

    wit_bindgen::generate!({
        with: {
            "wadge-test:fib/fib@0.1.0": generate,
            "wadge-test:leftpad/leftpad@0.1.0": generate,
            "wasi:io/error@0.2.1": wasi_passthrough::bindings::exports::wasi::io::error,
            "wasi:io/poll@0.2.1": wasi_passthrough::bindings::exports::wasi::io::poll,
            "wasi:io/streams@0.2.1": wasi_passthrough::bindings::exports::wasi::io::streams,
        }
    });
    export!(Handler);
}

use core::iter::{self, zip};

use wasi_passthrough::bindings::wasi::io::streams::{InputStream, OutputStream};
use wasi_passthrough_ext as _;

pub struct Handler;

impl bindings::exports::wadge_test::fib::fib::Guest for Handler {
    fn fib(n: u32) -> u64 {
        match n {
            0 => 0,
            1 | 2 => 1,
            n => Self::fib(n - 2) + Self::fib(n - 1),
        }
    }
}

impl bindings::exports::wadge_test::leftpad::leftpad::Guest for Handler {
    fn leftpad(
        in_: wasi_passthrough::bindings::exports::wasi::io::streams::InputStream,
        out: wasi_passthrough::bindings::exports::wasi::io::streams::OutputStreamBorrow<'_>,
        len: u64,
        c: char,
    ) -> Result<(), wasi_passthrough::bindings::exports::wasi::io::streams::StreamError> {
        let rx: InputStream = in_.into_inner();
        let tx: &OutputStream = out.get();

        let mut cs = zip(0..len, iter::repeat(c)).flat_map(|(_, c)| String::from(c).into_bytes());
        let mut buf = Vec::default();
        loop {
            let mut n = tx.check_write()?;
            if n == 0 {
                tx.subscribe().block();
                n = tx.check_write()?;
            }
            buf.extend(cs.by_ref().take(n.try_into().unwrap_or(usize::MAX)));
            if buf.is_empty() {
                break;
            }
            tx.write(&buf)?;
            buf.clear();
        }
        loop {
            let n = tx.splice(&rx, 4096)?;
            if n == 0 {
                return Ok(());
            }
        }
    }
}

use wasi::clocks::monotonic_clock;
use wasi::clocks::monotonic_clock::Duration;
use wasi::clocks::monotonic_clock::Instant;

use crate::bindings::{exports, wasi};
use crate::Handler;

impl exports::wasi::clocks::monotonic_clock::Guest for Handler {
    fn now() -> Instant {
        monotonic_clock::now()
    }

    fn resolution() -> Duration {
        monotonic_clock::resolution()
    }

    fn subscribe_instant(when: Instant) -> exports::wasi::io::poll::Pollable {
        exports::wasi::io::poll::Pollable::new(monotonic_clock::subscribe_instant(when))
    }

    fn subscribe_duration(when: Duration) -> exports::wasi::io::poll::Pollable {
        exports::wasi::io::poll::Pollable::new(monotonic_clock::subscribe_duration(when))
    }
}

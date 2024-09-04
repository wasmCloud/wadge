mod bindings {
    use crate::Handler;

    wit_bindgen::generate!({
        world: "host",
        path: "../../wit/fib",
        with: {
            "west-test:fib/fib": generate,
        }
    });
    export!(Handler);
}

use west_passthrough as _;

pub struct Handler;

impl bindings::exports::west_test::fib::fib::Guest for Handler {
    fn fib(n: u32) -> u64 {
        match n {
            0 => 0,
            1 | 2 => 1,
            n => Self::fib(n - 2) + Self::fib(n - 1),
        }
    }
}

use west::test::http_test;

use crate::bindings::{exports, west};
use crate::Handler;

impl exports::west::test::http_test::Guest for Handler {
    fn new_response_outparam() -> (
        exports::wasi::http::types::ResponseOutparam,
        exports::wasi::http::types::FutureIncomingResponse,
    ) {
        let (out, res) = http_test::new_response_outparam();
        (
            exports::wasi::http::types::ResponseOutparam::new(out),
            exports::wasi::http::types::FutureIncomingResponse::new(res),
        )
    }

    fn new_incoming_request(
        req: exports::wasi::http::types::OutgoingRequest,
    ) -> exports::wasi::http::types::IncomingRequest {
        exports::wasi::http::types::IncomingRequest::new(http_test::new_incoming_request(
            req.into_inner(),
        ))
    }
}

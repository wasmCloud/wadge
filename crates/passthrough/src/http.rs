use exports::wasi::http::types::IoErrorBorrow;

use wasi::http::types::Duration;
use wasi::http::types::{
    DnsErrorPayload, ErrorCode, FieldSizePayload, Fields, FutureIncomingResponse, FutureTrailers,
    HeaderError, IncomingBody, IncomingRequest, IncomingResponse, Method, OutgoingBody,
    OutgoingRequest, OutgoingResponse, RequestOptions, ResponseOutparam, Scheme,
    TlsAlertReceivedPayload,
};

use crate::bindings::{exports, wasi};
use crate::Handler;

impl From<Scheme> for exports::wasi::http::types::Scheme {
    fn from(value: Scheme) -> Self {
        match value {
            Scheme::Http => Self::Http,
            Scheme::Https => Self::Https,
            Scheme::Other(s) => Self::Other(s),
        }
    }
}

impl From<exports::wasi::http::types::Scheme> for Scheme {
    fn from(value: exports::wasi::http::types::Scheme) -> Self {
        match value {
            exports::wasi::http::types::Scheme::Http => Self::Http,
            exports::wasi::http::types::Scheme::Https => Self::Https,
            exports::wasi::http::types::Scheme::Other(s) => Self::Other(s),
        }
    }
}

impl From<HeaderError> for exports::wasi::http::types::HeaderError {
    fn from(value: HeaderError) -> Self {
        match value {
            HeaderError::InvalidSyntax => Self::InvalidSyntax,
            HeaderError::Forbidden => Self::Forbidden,
            HeaderError::Immutable => Self::Immutable,
        }
    }
}

impl From<exports::wasi::http::types::HeaderError> for HeaderError {
    fn from(value: exports::wasi::http::types::HeaderError) -> Self {
        match value {
            exports::wasi::http::types::HeaderError::InvalidSyntax => Self::InvalidSyntax,
            exports::wasi::http::types::HeaderError::Forbidden => Self::Forbidden,
            exports::wasi::http::types::HeaderError::Immutable => Self::Immutable,
        }
    }
}

impl From<Method> for exports::wasi::http::types::Method {
    fn from(value: Method) -> Self {
        match value {
            Method::Get => Self::Get,
            Method::Head => Self::Head,
            Method::Post => Self::Post,
            Method::Put => Self::Put,
            Method::Delete => Self::Delete,
            Method::Connect => Self::Connect,
            Method::Options => Self::Options,
            Method::Trace => Self::Trace,
            Method::Patch => Self::Patch,
            Method::Other(s) => Self::Other(s),
        }
    }
}

impl From<exports::wasi::http::types::Method> for Method {
    fn from(value: exports::wasi::http::types::Method) -> Self {
        match value {
            exports::wasi::http::types::Method::Get => Self::Get,
            exports::wasi::http::types::Method::Head => Self::Head,
            exports::wasi::http::types::Method::Post => Self::Post,
            exports::wasi::http::types::Method::Put => Self::Put,
            exports::wasi::http::types::Method::Delete => Self::Delete,
            exports::wasi::http::types::Method::Connect => Self::Connect,
            exports::wasi::http::types::Method::Options => Self::Options,
            exports::wasi::http::types::Method::Trace => Self::Trace,
            exports::wasi::http::types::Method::Patch => Self::Patch,
            exports::wasi::http::types::Method::Other(s) => Self::Other(s),
        }
    }
}

impl From<exports::wasi::http::types::ErrorCode> for wasi::http::types::ErrorCode {
    fn from(value: exports::wasi::http::types::ErrorCode) -> Self {
        match value {
            exports::wasi::http::types::ErrorCode::DnsTimeout => Self::DnsTimeout,
            exports::wasi::http::types::ErrorCode::DnsError(
                exports::wasi::http::types::DnsErrorPayload { rcode, info_code },
            ) => Self::DnsError(DnsErrorPayload { rcode, info_code }),
            exports::wasi::http::types::ErrorCode::DestinationNotFound => Self::DestinationNotFound,
            exports::wasi::http::types::ErrorCode::DestinationUnavailable => {
                Self::DestinationUnavailable
            }
            exports::wasi::http::types::ErrorCode::DestinationIpProhibited => {
                Self::DestinationIpProhibited
            }
            exports::wasi::http::types::ErrorCode::DestinationIpUnroutable => {
                Self::DestinationIpUnroutable
            }
            exports::wasi::http::types::ErrorCode::ConnectionRefused => Self::ConnectionRefused,
            exports::wasi::http::types::ErrorCode::ConnectionTerminated => {
                Self::ConnectionTerminated
            }
            exports::wasi::http::types::ErrorCode::ConnectionTimeout => Self::ConnectionTimeout,
            exports::wasi::http::types::ErrorCode::ConnectionReadTimeout => {
                Self::ConnectionReadTimeout
            }
            exports::wasi::http::types::ErrorCode::ConnectionWriteTimeout => {
                Self::ConnectionWriteTimeout
            }
            exports::wasi::http::types::ErrorCode::ConnectionLimitReached => {
                Self::ConnectionLimitReached
            }
            exports::wasi::http::types::ErrorCode::TlsProtocolError => Self::TlsProtocolError,
            exports::wasi::http::types::ErrorCode::TlsCertificateError => Self::TlsCertificateError,
            exports::wasi::http::types::ErrorCode::TlsAlertReceived(
                exports::wasi::http::types::TlsAlertReceivedPayload {
                    alert_id,
                    alert_message,
                },
            ) => Self::TlsAlertReceived(TlsAlertReceivedPayload {
                alert_id,
                alert_message,
            }),
            exports::wasi::http::types::ErrorCode::HttpRequestDenied => Self::HttpRequestDenied,
            exports::wasi::http::types::ErrorCode::HttpRequestLengthRequired => {
                Self::HttpRequestLengthRequired
            }
            exports::wasi::http::types::ErrorCode::HttpRequestBodySize(s) => {
                Self::HttpRequestBodySize(s)
            }
            exports::wasi::http::types::ErrorCode::HttpRequestMethodInvalid => {
                Self::HttpRequestMethodInvalid
            }
            exports::wasi::http::types::ErrorCode::HttpRequestUriInvalid => {
                Self::HttpRequestUriInvalid
            }
            exports::wasi::http::types::ErrorCode::HttpRequestUriTooLong => {
                Self::HttpRequestUriTooLong
            }
            exports::wasi::http::types::ErrorCode::HttpRequestHeaderSectionSize(s) => {
                Self::HttpRequestHeaderSectionSize(s)
            }
            exports::wasi::http::types::ErrorCode::HttpRequestHeaderSize(Some(
                exports::wasi::http::types::FieldSizePayload {
                    field_name,
                    field_size,
                },
            )) => Self::HttpRequestHeaderSize(Some(FieldSizePayload {
                field_name,
                field_size,
            })),
            exports::wasi::http::types::ErrorCode::HttpRequestHeaderSize(None) => {
                Self::HttpRequestHeaderSize(None)
            }
            exports::wasi::http::types::ErrorCode::HttpRequestTrailerSectionSize(s) => {
                Self::HttpRequestTrailerSectionSize(s)
            }
            exports::wasi::http::types::ErrorCode::HttpRequestTrailerSize(
                exports::wasi::http::types::FieldSizePayload {
                    field_name,
                    field_size,
                },
            ) => Self::HttpRequestTrailerSize(FieldSizePayload {
                field_name,
                field_size,
            }),
            exports::wasi::http::types::ErrorCode::HttpResponseIncomplete => {
                Self::HttpResponseIncomplete
            }
            exports::wasi::http::types::ErrorCode::HttpResponseHeaderSectionSize(s) => {
                Self::HttpResponseHeaderSectionSize(s)
            }
            exports::wasi::http::types::ErrorCode::HttpResponseHeaderSize(
                exports::wasi::http::types::FieldSizePayload {
                    field_name,
                    field_size,
                },
            ) => Self::HttpResponseHeaderSize(FieldSizePayload {
                field_name,
                field_size,
            }),
            exports::wasi::http::types::ErrorCode::HttpResponseBodySize(s) => {
                Self::HttpResponseBodySize(s)
            }
            exports::wasi::http::types::ErrorCode::HttpResponseTrailerSectionSize(s) => {
                Self::HttpResponseTrailerSectionSize(s)
            }
            exports::wasi::http::types::ErrorCode::HttpResponseTrailerSize(
                exports::wasi::http::types::FieldSizePayload {
                    field_name,
                    field_size,
                },
            ) => Self::HttpResponseTrailerSize(FieldSizePayload {
                field_name,
                field_size,
            }),
            exports::wasi::http::types::ErrorCode::HttpResponseTransferCoding(e) => {
                Self::HttpResponseTransferCoding(e)
            }
            exports::wasi::http::types::ErrorCode::HttpResponseContentCoding(e) => {
                Self::HttpResponseContentCoding(e)
            }
            exports::wasi::http::types::ErrorCode::HttpResponseTimeout => Self::HttpResponseTimeout,
            exports::wasi::http::types::ErrorCode::HttpUpgradeFailed => Self::HttpUpgradeFailed,
            exports::wasi::http::types::ErrorCode::HttpProtocolError => Self::HttpProtocolError,
            exports::wasi::http::types::ErrorCode::LoopDetected => Self::LoopDetected,
            exports::wasi::http::types::ErrorCode::ConfigurationError => Self::ConfigurationError,
            exports::wasi::http::types::ErrorCode::InternalError(e) => Self::InternalError(e),
        }
    }
}

impl From<ErrorCode> for exports::wasi::http::types::ErrorCode {
    fn from(value: ErrorCode) -> Self {
        match value {
            ErrorCode::DnsTimeout => Self::DnsTimeout,
            ErrorCode::DnsError(DnsErrorPayload { rcode, info_code }) => {
                Self::DnsError(exports::wasi::http::types::DnsErrorPayload { rcode, info_code })
            }
            ErrorCode::DestinationNotFound => Self::DestinationNotFound,
            ErrorCode::DestinationUnavailable => Self::DestinationUnavailable,
            ErrorCode::DestinationIpProhibited => Self::DestinationIpProhibited,
            ErrorCode::DestinationIpUnroutable => Self::DestinationIpUnroutable,
            ErrorCode::ConnectionRefused => Self::ConnectionRefused,
            ErrorCode::ConnectionTerminated => Self::ConnectionTerminated,
            ErrorCode::ConnectionTimeout => Self::ConnectionTimeout,
            ErrorCode::ConnectionReadTimeout => Self::ConnectionReadTimeout,
            ErrorCode::ConnectionWriteTimeout => Self::ConnectionWriteTimeout,
            ErrorCode::ConnectionLimitReached => Self::ConnectionLimitReached,
            ErrorCode::TlsProtocolError => Self::TlsProtocolError,
            ErrorCode::TlsCertificateError => Self::TlsCertificateError,
            ErrorCode::TlsAlertReceived(TlsAlertReceivedPayload {
                alert_id,
                alert_message,
            }) => Self::TlsAlertReceived(exports::wasi::http::types::TlsAlertReceivedPayload {
                alert_id,
                alert_message,
            }),
            ErrorCode::HttpRequestDenied => Self::HttpRequestDenied,
            ErrorCode::HttpRequestLengthRequired => Self::HttpRequestLengthRequired,
            ErrorCode::HttpRequestBodySize(s) => Self::HttpRequestBodySize(s),
            ErrorCode::HttpRequestMethodInvalid => Self::HttpRequestMethodInvalid,
            ErrorCode::HttpRequestUriInvalid => Self::HttpRequestUriInvalid,
            ErrorCode::HttpRequestUriTooLong => Self::HttpRequestUriTooLong,
            ErrorCode::HttpRequestHeaderSectionSize(s) => Self::HttpRequestHeaderSectionSize(s),
            ErrorCode::HttpRequestHeaderSize(Some(FieldSizePayload {
                field_name,
                field_size,
            })) => {
                Self::HttpRequestHeaderSize(Some(exports::wasi::http::types::FieldSizePayload {
                    field_name,
                    field_size,
                }))
            }
            ErrorCode::HttpRequestHeaderSize(None) => Self::HttpRequestHeaderSize(None),
            ErrorCode::HttpRequestTrailerSectionSize(s) => Self::HttpRequestTrailerSectionSize(s),
            ErrorCode::HttpRequestTrailerSize(FieldSizePayload {
                field_name,
                field_size,
            }) => Self::HttpRequestTrailerSize(exports::wasi::http::types::FieldSizePayload {
                field_name,
                field_size,
            }),
            ErrorCode::HttpResponseIncomplete => Self::HttpResponseIncomplete,
            ErrorCode::HttpResponseHeaderSectionSize(s) => Self::HttpResponseHeaderSectionSize(s),
            ErrorCode::HttpResponseHeaderSize(FieldSizePayload {
                field_name,
                field_size,
            }) => Self::HttpResponseHeaderSize(exports::wasi::http::types::FieldSizePayload {
                field_name,
                field_size,
            }),
            ErrorCode::HttpResponseBodySize(s) => Self::HttpResponseBodySize(s),
            ErrorCode::HttpResponseTrailerSectionSize(s) => Self::HttpResponseTrailerSectionSize(s),
            ErrorCode::HttpResponseTrailerSize(FieldSizePayload {
                field_name,
                field_size,
            }) => Self::HttpResponseTrailerSize(exports::wasi::http::types::FieldSizePayload {
                field_name,
                field_size,
            }),
            ErrorCode::HttpResponseTransferCoding(e) => Self::HttpResponseTransferCoding(e),
            ErrorCode::HttpResponseContentCoding(e) => Self::HttpResponseContentCoding(e),
            ErrorCode::HttpResponseTimeout => Self::HttpResponseTimeout,
            ErrorCode::HttpUpgradeFailed => Self::HttpUpgradeFailed,
            ErrorCode::HttpProtocolError => Self::HttpProtocolError,
            ErrorCode::LoopDetected => Self::LoopDetected,
            ErrorCode::ConfigurationError => Self::ConfigurationError,
            ErrorCode::InternalError(e) => Self::InternalError(e),
        }
    }
}

impl exports::wasi::http::outgoing_handler::Guest for Handler {
    fn handle(
        request: exports::wasi::http::types::OutgoingRequest,
        options: Option<exports::wasi::http::types::RequestOptions>,
    ) -> Result<
        exports::wasi::http::types::FutureIncomingResponse,
        exports::wasi::http::types::ErrorCode,
    > {
        todo!()
    }
}

impl exports::wasi::http::types::Guest for Handler {
    type Fields = Fields;
    type IncomingRequest = IncomingRequest;
    type OutgoingRequest = OutgoingRequest;
    type RequestOptions = RequestOptions;
    type ResponseOutparam = ResponseOutparam;
    type IncomingResponse = IncomingResponse;
    type IncomingBody = IncomingBody;
    type FutureTrailers = FutureTrailers;
    type OutgoingResponse = OutgoingResponse;
    type OutgoingBody = OutgoingBody;
    type FutureIncomingResponse = FutureIncomingResponse;

    fn http_error_code(err: IoErrorBorrow<'_>) -> Option<exports::wasi::http::types::ErrorCode> {
        todo!()
    }
}

impl exports::wasi::http::types::GuestFields for wasi::http::types::Fields {
    fn new() -> Self {
        Self::new()
    }

    fn get(&self, name: String) -> Vec<Vec<u8>> {
        Self::get(self, &name)
    }

    fn set(
        &self,
        name: String,
        value: Vec<Vec<u8>>,
    ) -> Result<(), exports::wasi::http::types::HeaderError> {
        Self::set(self, &name, &value)?;
        Ok(())
    }

    fn delete(&self, name: String) -> Result<(), exports::wasi::http::types::HeaderError> {
        Self::delete(self, &name)?;
        Ok(())
    }

    fn append(
        &self,
        name: String,
        value: Vec<u8>,
    ) -> Result<(), exports::wasi::http::types::HeaderError> {
        Self::append(self, &name, &value)?;
        Ok(())
    }

    fn entries(&self) -> Vec<(String, Vec<u8>)> {
        Self::entries(self)
    }

    fn clone(&self) -> exports::wasi::http::types::Fields {
        exports::wasi::http::types::Fields::new(Self::clone(self))
    }

    fn from_list(
        entries: Vec<(String, Vec<u8>)>,
    ) -> Result<exports::wasi::http::types::Fields, exports::wasi::http::types::HeaderError> {
        let ret = Self::from_list(&entries)?;
        Ok(exports::wasi::http::types::Fields::new(ret))
    }

    fn has(&self, name: String) -> bool {
        Self::has(self, &name)
    }
}

impl exports::wasi::http::types::GuestIncomingRequest for IncomingRequest {
    fn method(&self) -> exports::wasi::http::types::Method {
        Self::method(self).into()
    }
    fn path_with_query(&self) -> Option<String> {
        Self::path_with_query(self)
    }
    fn scheme(&self) -> Option<exports::wasi::http::types::Scheme> {
        Self::scheme(self).map(Into::into)
    }
    fn authority(&self) -> Option<String> {
        Self::authority(self)
    }
    fn headers(&self) -> exports::wasi::http::types::Fields {
        exports::wasi::http::types::Fields::new(Self::headers(self))
    }
    fn consume(&self) -> Result<exports::wasi::http::types::IncomingBody, ()> {
        let ret = Self::consume(self)?;
        Ok(exports::wasi::http::types::IncomingBody::new(ret))
    }
}

impl exports::wasi::http::types::GuestOutgoingRequest for OutgoingRequest {
    fn new(headers: exports::wasi::http::types::Headers) -> Self {
        Self::new(headers.into_inner())
    }

    fn body(&self) -> Result<exports::wasi::http::types::OutgoingBody, ()> {
        let ret = Self::body(self)?;
        Ok(exports::wasi::http::types::OutgoingBody::new(ret))
    }

    fn method(&self) -> exports::wasi::http::types::Method {
        Self::method(self).into()
    }

    fn set_method(&self, method: exports::wasi::http::types::Method) -> Result<(), ()> {
        Self::set_method(self, &method.into())
    }

    fn path_with_query(&self) -> Option<String> {
        Self::path_with_query(self)
    }

    fn set_path_with_query(&self, path_with_query: Option<String>) -> Result<(), ()> {
        Self::set_path_with_query(self, path_with_query.as_deref())
    }

    fn scheme(&self) -> Option<exports::wasi::http::types::Scheme> {
        Self::scheme(self).map(Into::into)
    }

    fn set_scheme(&self, scheme: Option<exports::wasi::http::types::Scheme>) -> Result<(), ()> {
        Self::set_scheme(self, scheme.map(Into::into).as_ref())
    }

    fn authority(&self) -> Option<String> {
        Self::authority(self)
    }

    fn set_authority(&self, authority: Option<String>) -> Result<(), ()> {
        Self::set_authority(self, authority.as_deref())
    }

    fn headers(&self) -> exports::wasi::http::types::Headers {
        exports::wasi::http::types::Headers::new(Self::headers(self))
    }
}

impl exports::wasi::http::types::GuestRequestOptions for RequestOptions {
    fn new() -> Self {
        Self::new()
    }
    fn connect_timeout(&self) -> Option<Duration> {
        Self::connect_timeout(self)
    }
    fn set_connect_timeout(&self, duration: Option<Duration>) -> Result<(), ()> {
        Self::set_connect_timeout(self, duration)
    }
    fn first_byte_timeout(&self) -> Option<Duration> {
        Self::first_byte_timeout(self)
    }
    fn set_first_byte_timeout(&self, duration: Option<Duration>) -> Result<(), ()> {
        Self::set_first_byte_timeout(self, duration)
    }
    fn between_bytes_timeout(&self) -> Option<Duration> {
        Self::between_bytes_timeout(self)
    }
    fn set_between_bytes_timeout(&self, duration: Option<Duration>) -> Result<(), ()> {
        Self::set_between_bytes_timeout(self, duration)
    }
}

impl exports::wasi::http::types::GuestResponseOutparam for ResponseOutparam {
    fn set(
        param: exports::wasi::http::types::ResponseOutparam,
        response: Result<
            exports::wasi::http::types::OutgoingResponse,
            exports::wasi::http::types::ErrorCode,
        >,
    ) {
        Self::set(
            param.into_inner(),
            response
                .map(exports::wasi::http::types::OutgoingResponse::into_inner)
                .map_err(Into::into),
        );
    }
}

impl exports::wasi::http::types::GuestIncomingResponse for IncomingResponse {
    fn status(&self) -> exports::wasi::http::types::StatusCode {
        Self::status(self)
    }
    fn headers(&self) -> exports::wasi::http::types::Fields {
        exports::wasi::http::types::Fields::new(Self::headers(self))
    }
    fn consume(&self) -> Result<exports::wasi::http::types::IncomingBody, ()> {
        Self::consume(self).map(exports::wasi::http::types::IncomingBody::new)
    }
}

impl exports::wasi::http::types::GuestIncomingBody for IncomingBody {
    fn stream(&self) -> Result<exports::wasi::io::streams::InputStream, ()> {
        Self::stream(self).map(exports::wasi::io::streams::InputStream::new)
    }

    fn finish(
        body: exports::wasi::http::types::IncomingBody,
    ) -> exports::wasi::http::types::FutureTrailers {
        exports::wasi::http::types::FutureTrailers::new(Self::finish(body.into_inner()))
    }
}

impl exports::wasi::http::types::GuestFutureTrailers for FutureTrailers {
    fn subscribe(&self) -> exports::wasi::io::poll::Pollable {
        exports::wasi::io::poll::Pollable::new(Self::subscribe(self))
    }

    fn get(
        &self,
    ) -> Option<
        Result<
            Result<
                Option<exports::wasi::http::types::Fields>,
                exports::wasi::http::types::ErrorCode,
            >,
            (),
        >,
    > {
        match Self::get(self)? {
            Ok(Ok(Some(fields))) => Some(Ok(Ok(Some(exports::wasi::http::types::Fields::new(
                fields,
            ))))),
            Ok(Ok(None)) => Some(Ok(Ok(None))),
            Ok(Err(code)) => Some(Ok(Err(code.into()))),
            Err(()) => Some(Err(())),
        }
    }
}

impl exports::wasi::http::types::GuestOutgoingResponse for OutgoingResponse {
    fn new(headers: exports::wasi::http::types::Fields) -> Self {
        Self::new(headers.into_inner())
    }

    fn status_code(&self) -> exports::wasi::http::types::StatusCode {
        Self::status_code(self)
    }

    fn set_status_code(
        &self,
        status_code: exports::wasi::http::types::StatusCode,
    ) -> Result<(), ()> {
        Self::set_status_code(self, status_code)
    }

    fn headers(&self) -> exports::wasi::http::types::Headers {
        exports::wasi::http::types::Headers::new(Self::headers(self))
    }

    fn body(&self) -> Result<exports::wasi::http::types::OutgoingBody, ()> {
        let ret = Self::body(self)?;
        Ok(exports::wasi::http::types::OutgoingBody::new(ret))
    }
}

impl exports::wasi::http::types::GuestOutgoingBody for OutgoingBody {
    fn write(&self) -> Result<exports::wasi::io::streams::OutputStream, ()> {
        let ret = Self::write(self)?;
        Ok(exports::wasi::io::streams::OutputStream::new(ret))
    }

    fn finish(
        body: exports::wasi::http::types::OutgoingBody,
        trailers: Option<exports::wasi::http::types::Fields>,
    ) -> Result<(), exports::wasi::http::types::ErrorCode> {
        Self::finish(
            body.into_inner(),
            trailers.map(exports::wasi::http::types::Fields::into_inner),
        )?;
        Ok(())
    }
}

impl exports::wasi::http::types::GuestFutureIncomingResponse for FutureIncomingResponse {
    fn subscribe(&self) -> exports::wasi::io::poll::Pollable {
        exports::wasi::io::poll::Pollable::new(Self::subscribe(self))
    }

    fn get(
        &self,
    ) -> Option<
        Result<
            Result<
                exports::wasi::http::types::IncomingResponse,
                exports::wasi::http::types::ErrorCode,
            >,
            (),
        >,
    > {
        match Self::get(self)? {
            Ok(Ok(res)) => Some(Ok(Ok(exports::wasi::http::types::IncomingResponse::new(
                res,
            )))),
            Ok(Err(code)) => Some(Ok(Err(code.into()))),
            Err(()) => Some(Err(())),
        }
    }
}

#![doc = include_str!("../README.md")]

mod error;
mod request;
mod response;
mod transport;
mod version;

pub use self::error::Error;
pub use self::error::ErrorKind;
pub use self::error::Result;

pub use self::request::BoxRequest;
pub use self::request::Request;
pub use self::request::RequestBatch;
pub use self::request::RequestHeader;
pub use self::request::RequestId;

pub use self::response::BoxResponse;
pub use self::response::Response;
pub use self::response::ResponseBatch;
pub use self::response::ResponseError;
pub use self::response::ResponseFailure;
pub use self::response::ResponseSuccess;

pub use self::transport::RpcClient;
pub use self::transport::RpcTransport;

pub use self::version::VERSION;

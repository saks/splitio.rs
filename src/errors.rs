use failure_derive::Fail;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Cache adapter is not connected")]
    NotConnected,
    #[fail(display = "Cannot deserialize split data: `{}'", _0)]
    ParseError(#[fail(cause)] serde_json::Error),
    #[fail(display = "Cannot read cache from redis: `{}'", _0)]
    RedisError(#[fail(cause)] redis::RedisError),
}

pub type SplitResult<T> = std::result::Result<T, Error>;

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::ParseError(e)
    }
}

impl From<redis::RedisError> for Error {
    fn from(e: redis::RedisError) -> Self {
        dbg!(&e);
        Error::RedisError(e)
    }
}

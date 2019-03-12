use crate::errors::SplitResult;
use crate::split::Split;
use log::debug;
use redis;
use redis::Commands as _;

pub trait Cache {
    fn get(&self, key: &str) -> SplitResult<Option<Split>>;
}

#[derive(Debug, serde::Deserialize)]
pub struct File {
    splits: Vec<Split>,
}

impl File {
    pub fn from_path(file_name: &str) -> SplitResult<Self> {
        use std::fs::File;
        use std::io::Read as _;

        let mut file = File::open(file_name).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        Ok(serde_json::from_str(&content)?)
    }
}

impl Cache for File {
    fn get(&self, split_name: &str) -> SplitResult<Option<Split>> {
        let split = self.splits.iter().find(|s| s.name == split_name);

        Ok(match split {
            Some(s) => Some(s.clone()),
            None => None,
        })
    }
}

use std::sync::Mutex;
pub struct Redis {
    connection: Mutex<redis::Connection>,
}

impl Redis {
    pub fn from_url(url: &str) -> SplitResult<Self> {
        debug!("connecting to redis...");
        let client = redis::Client::open(url)?;
        let connection = client.get_connection()?;
        let connection = Mutex::new(connection);

        Ok(Redis { connection })
    }
}

impl Cache for Redis {
    fn get(&self, split_name: &str) -> SplitResult<Option<Split>> {
        let redis_key = build_split_key(split_name);
        let connection = self.connection.lock()?;
        let value: Option<String> = connection.get(redis_key)?;

        match value {
            Some(json) => {
                let split = serde_json::from_str(&json)?;
                Ok(Some(split))
            }
            None => Ok(None),
        }
    }
}

const REDIS_CACHE_PREFIX: &str = "SPLITIO";
fn build_split_key(split_name: &str) -> String {
    format!(
        "{prefix}.split.{split_name}",
        prefix = REDIS_CACHE_PREFIX,
        split_name = split_name
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Redis URL did not parse")]
    fn bad_redis_url() {
        Redis::from_url("foo").unwrap();
    }

    #[test]
    #[should_panic(expected = "RedisError(Connection refused (os error 111))")]
    fn no_redis() {
        Redis::from_url("redis://127.0.0.1:80").unwrap();
    }

    #[test]
    fn split_redis_key() {
        assert_eq!("SPLITIO.split.foo", build_split_key("foo"));
    }
}

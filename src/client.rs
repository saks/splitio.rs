use super::Treatment;
use crate::errors::SplitResult;
use crate::storage::CacheAdapter;

pub struct Client<'a> {
    cache: &'a CacheAdapter,
}

impl<'a> Client<'a> {
    pub fn new(cache: &'a impl CacheAdapter) -> Self {
        Self { cache }
    }

    pub fn get_treatment(&self, key: &str, split_name: &str) -> SplitResult<Treatment> {
        let split = match self.cache.get(split_name)? {
            Some(s) => s,
            None => {
                return Ok(Treatment::Control);
            }
        };

        Ok(split.get_treatment(key))
    }
}

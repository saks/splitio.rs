use super::Treatment;
use crate::cache::Cache;
use crate::errors::SplitResult;

pub struct Client<'a, T: Cache> {
    cache: Box<&'a T>,
}

// pub type Attrs = std::collections::HashMap<String, i64>;

impl<'a, T: Cache> Client<'a, T> {
    pub fn new(cache: &'a T) -> Self {
        Self {
            cache: Box::new(cache),
        }
    }

    pub fn get_treatment(
        &self,
        key: &str,
        split_name: &str, // attrs: Option<Attrs>
    ) -> SplitResult<Treatment> {
        let split = match self.cache.get(split_name)? {
            Some(s) => s,
            None => {
                return Ok(Treatment::Control);
            }
        };

        Ok(split.get_treatment(key))
    }
}

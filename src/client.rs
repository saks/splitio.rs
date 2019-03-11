use super::Treatment;
use crate::errors::SplitResult;
use crate::storage::CacheAdapter;

// pub struct Client<'a> {
//     cache: &'a CacheAdapter,
// }
pub struct Client;

// pub type Attrs = std::collections::HashMap<String, i64>;

// impl<'a> Client<'a> {
impl Client {
    pub fn new() -> Self {
        Self {}
    }
    // pub fn new(cache: &'a impl CacheAdapter) -> Self {
    //     Self { cache }
    // }

    pub fn get_treatment(
        &self,
        key: &str,
        split_name: &str,
        cache: &impl CacheAdapter, // attrs: Option<Attrs>
    ) -> SplitResult<Treatment> {
        let split = match cache.get(split_name)? {
            Some(s) => s,
            None => {
                return Ok(Treatment::Control);
            }
        };

        Ok(split.get_treatment(key))
    }
}

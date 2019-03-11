use crate::splitter;
use crate::{condition::Condition, Treatment};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Split {
    pub name: String,
    default_treatment: Treatment,
    killed: bool,
    traffic_allocation: Option<u8>,
    seed: i64,
    status: Status,
    conditions: Vec<Condition>,
    change_number: Option<u64>,
}

impl Split {
    pub fn is_garbage(&self) -> bool {
        self.status == Status::Archived
    }

    pub fn get_treatment(&self, key: &str) -> Treatment {
        if self.is_garbage() {
            Treatment::Control
        } else if self.killed {
            self.default_treatment
        } else {
            self.evaluate(key)
        }
    }

    fn evaluate(&self, key: &str) -> Treatment {
        use crate::condition::ConditionType;

        let mut in_rollout = false;

        for condition in &self.conditions {
            if condition.is_empty() {
                continue;
            };

            if !in_rollout && condition.condition_type == Some(ConditionType::Rollout) {
                if self.traffic_allocation.unwrap() < 100 {
                    // TODO
                    // bucket = splitter.bucket(splitter.count_hash(key, split[:trafficAllocationSeed].to_i, legacy_algo))
                    //
                    // if bucket > split[:trafficAllocation]
                    //     return treatment_hash(Models::Label::NOT_IN_SPLIT, split[:defaultTreatment], split[:changeNumber])
                    // end
                }
            };
            in_rollout = true;

            if !condition.is_match(key) {
                continue;
            }

            return splitter::get_treatment(key, self.seed, &condition);
        }

        self.default_treatment
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum Status {
    Active,
    Archived,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Client;
    use std::collections::HashMap;

    // Tests that get_treatment returns control treatment if feature is unknown
    #[test]
    fn unknown_feature_returns_control() {
        let c = cache("splitChanges");
        let client = Client::new(&c);

        assert_eq!(
            Treatment::Control,
            client
                .get_treatment(SOME_KEY, UNKNOWN_FEATURE_NAME)
                .unwrap()
        );
    }

    //----- START TESTS FROM RUBY CLIENT

    macro_rules! assert_treatment_is_on {
        ($file_name:literal, $id:literal, $feature:literal) => {
            let c = cache($file_name);
            let client = Client::new(&c);

            assert_eq!(Treatment::On, client.get_treatment($id, $feature).unwrap());
        };
    }

    macro_rules! assert_treatment_is_off {
        ($file_name:literal, $id:literal, $feature:literal) => {
            let c = cache($file_name);
            let client = Client::new(&c);

            assert_eq!(Treatment::Off, client.get_treatment($id, $feature).unwrap());
        };
    }

    // Validates the feature is on for all ids
    #[test]
    fn all_keys_matcher_feature_is_on_for_all_ids() {
        assert_treatment_is_on!("all_keys_matcher", "fake_user_id_1", "test_feature");
        assert_treatment_is_on!("all_keys_matcher", "fake_user_id_2", "test_feature");
    }

    #[test]
    fn whitelist_matcher() {
        assert_treatment_is_on!("whitelist_matcher", "fake_user_id_1", "test_whitelist");
        assert_treatment_is_off!("whitelist_matcher", "fake_user_id_2", "test_whitelist");
    }

    // Test that get_treatment returns on for the test_between_datetime feature using the user key
    // included for on treatment
    // TODO
    // #[test]
    // fn between_datetime_include_on_user() {
    //     let c = cache("splitChanges");
    //     let client = Client::new(&c);
    //     let mut attrs = HashMap::new();
    //     attrs.insert(ATTRIBUTE_NAME.into(), in_between_datetime());
    //
    //     assert_eq!(
    //         Treatment::On,
    //         client
    //             .get_treatment(FAKE_ID_ON_KEY, "test_between_datetime")
    //             .unwrap()
    //     );
    // }

    use crate::storage::CacheAdapter;
    fn cache(name: &str) -> impl CacheAdapter {
        let path = format!("test_data/{}.json", name);
        crate::storage::FileCacheAdapter::from_path(&path).unwrap()
    }

    fn in_between_datetime() -> i64 {
        use chrono::prelude::*;
        chrono::Utc.ymd(2016, 4, 25).and_hms(16, 0, 0).timestamp()
    }
    const ATTRIBUTE_NAME: &str = "some_attribute";
    const SOME_KEY: &str = "some_key";
    const UNKNOWN_FEATURE_NAME: &str = "foobar";
    const FAKE_ID_ON_KEY: &str = "fake_id_on";
}

use crate::{condition::Condition, Treatment};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Split {
    pub name: String,
    default_treatment: Treatment,
    killed: bool,
    traffic_allocation: Option<u8>,
    status: Status,
    conditions: Vec<Condition>,
    change_number: u64,
}

impl Split {
    pub fn is_garbage(&self) -> bool {
        self.status == Status::Archived
    }

    pub fn get_treatment(&self, key: &str) -> Treatment {
        if self.is_garbage() {
            Treatment::Control
        } else if self.killed {
            self.default_treatment.clone()
        } else {
            self.evaluate(key)
        }
    }

    fn evaluate(&self, _key: &str) -> Treatment {
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
        }

        Treatment::Control
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
        let c = cache();
        let client = Client::new(&c);

        assert_eq!(
            Treatment::Control,
            client
                .get_treatment(SOME_KEY, UNKNOWN_FEATURE_NAME, None)
                .unwrap()
        );
    }

    // Test that get_treatment returns on for the test_between_datetime feature using the user key
    // included for on treatment
    #[test]
    fn between_datetime_include_on_user() {
        let c = cache();
        let client = Client::new(&c);
        let mut attrs = HashMap::new();
        attrs.insert(ATTRIBUTE_NAME.into(), in_between_datetime());

        assert_eq!(
            Treatment::On,
            client
                .get_treatment(FAKE_ID_ON_KEY, "test_between_datetime", Some(attrs))
                .unwrap()
        );

        // self.assertEqual(
        //     self.on_treatment,
        //     self.client.get_treatment(
        //         self.fake_id_on_key, 'test_between_datetime',
        //         {self.attribute_name: self.in_between_datetime}))
    }

    use crate::storage::CacheAdapter;
    fn cache() -> impl CacheAdapter {
        crate::storage::FileCacheAdapter::from_path("test_data/splitChanges.json").unwrap()
    }

    fn split() -> Split {
        use std::fs::File;
        use std::io::Read as _;

        let mut file = File::open("foo.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        serde_json::from_str(&contents).unwrap()
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

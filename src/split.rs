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

    // Tests that get_treatment returns control treatment if feature is unknown
    #[test]
    fn unknown_feature_returns_control() {
        let c = cache();
        let client = Client::new(&c);

        assert_eq!(
            Treatment::Control,
            client
                .get_treatment(SOME_KEY, UNKNOWN_FEATURE_NAME)
                .unwrap()
        );
    }

    // #[test]
    // fn deserialize_split() {
    //     serde_json::from_str::<Split>(JSON_1).unwrap();
    //     serde_json::from_str::<Split>(JSON_2).unwrap();
    //     dbg!(in_between_datetime());
    // }

    // Test that get_treatment returns on for the test_between_datetime feature using the user key
    // included for on treatment
    // #[test]
    // fn between_datetime_include_on_user() {
    //     assert_eq!(Treatment::On, )
    //         self.client.get_treatment(
    //             self.fake_id_on_key, 'test_between_datetime',
    //             {self.attribute_name: self.in_between_datetime}))
    //
    // }

    // Test that get_treatment returns off for the test_between_datetime feature using the some key
    // and no attributes
    // #[test]
    // fn between_datetime_some_key_no_attributes() {
    //     let c = cache();
    //     let client = Client::new(&c);
    //     assert_eq!(
    //         Treatment::Off,
    //         client
    //             .get_treatment(SOME_KEY, "test_between_datetime")
    //             .unwrap()
    //     );
    //     // self.assertEqual(
    //     //     self.off_treatment,
    //     //     self.client.get_treatment(self.some_key, 'test_between_datetime'))
    // }

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

    const JSON_1: &str = r#"{
          "trafficTypeName": "user",
          "name": "some_ui_rebranding",
          "trafficAllocation": 100,
          "trafficAllocationSeed": -1397198300,
          "seed": -1634554238,
          "status": "ACTIVE",
          "killed": false,
          "defaultTreatment": "off",
          "changeNumber": 1544477114146,
          "algo": 2,
          "conditions": [
            {
              "conditionType": "WHITELIST",
              "matcherGroup": {
                "combiner": "AND",
                "matchers": [
                  {
                    "keySelector": null,
                    "matcherType": "WHITELIST",
                    "negate": false,
                    "userDefinedSegmentMatcherData": null,
                    "whitelistMatcherData": {
                      "whitelist": [
                        "123",
                        "124"
                      ]
                    },
                    "unaryNumericMatcherData": null,
                    "betweenMatcherData": null,
                    "booleanMatcherData": null,
                    "dependencyMatcherData": null,
                    "stringMatcherData": null
                  }
                ]
              },
              "partitions": [
                {
                  "treatment": "on",
                  "size": 100
                }
              ],
              "label": "whitelisted"
            },
            {
              "conditionType": "WHITELIST",
              "matcherGroup": {
                "combiner": "AND",
                "matchers": [
                  {
                    "keySelector": null,
                    "matcherType": "WHITELIST",
                    "negate": false,
                    "userDefinedSegmentMatcherData": null,
                    "whitelistMatcherData": {
                      "whitelist": [
                        "125",
                        "126"
                      ]
                    },
                    "unaryNumericMatcherData": null,
                    "betweenMatcherData": null,
                    "booleanMatcherData": null,
                    "dependencyMatcherData": null,
                    "stringMatcherData": null
                  }
                ]
              },
              "partitions": [
                {
                  "treatment": "off",
                  "size": 100
                }
              ],
              "label": "whitelisted"
            },
            {
              "conditionType": "ROLLOUT",
              "matcherGroup": {
                "combiner": "AND",
                "matchers": [
                  {
                    "keySelector": {
                      "trafficType": "user",
                      "attribute": null
                    },
                    "matcherType": "ALL_KEYS",
                    "negate": false,
                    "userDefinedSegmentMatcherData": null,
                    "whitelistMatcherData": null,
                    "unaryNumericMatcherData": null,
                    "betweenMatcherData": null,
                    "booleanMatcherData": null,
                    "dependencyMatcherData": null,
                    "stringMatcherData": null
                  }
                ]
              },
              "partitions": [
                {
                  "treatment": "on",
                  "size": 100
                },
                {
                  "treatment": "off",
                  "size": 0
                }
              ],
              "label": "default rule"
            }
          ]
        }"#;
    const JSON_2: &str = r#"
        {
          "trafficTypeName": "user",
          "name": "some_email_redesign",
          "trafficAllocation": 100,
          "trafficAllocationSeed": 465979707,
          "seed": 45350536,
          "status": "ACTIVE",
          "killed": false,
          "defaultTreatment": "off",
          "changeNumber": 1547857786288,
          "algo": 2,
          "conditions": [
            {
              "conditionType": "WHITELIST",
              "matcherGroup": {
                "combiner": "AND",
                "matchers": [
                  {
                    "keySelector": null,
                    "matcherType": "WHITELIST",
                    "negate": false,
                    "userDefinedSegmentMatcherData": null,
                    "whitelistMatcherData": {
                      "whitelist": [
                        "123",
                        "124",
                        "125"
                      ]
                    },
                    "unaryNumericMatcherData": null,
                    "betweenMatcherData": null,
                    "booleanMatcherData": null,
                    "dependencyMatcherData": null,
                    "stringMatcherData": null
                  }
                ]
              },
              "partitions": [
                {
                  "treatment": "on",
                  "size": 100
                }
              ],
              "label": "whitelisted"
            },
            {
              "conditionType": "ROLLOUT",
              "matcherGroup": {
                "combiner": "AND",
                "matchers": [
                  {
                    "keySelector": {
                      "trafficType": "user",
                      "attribute": null
                    },
                    "matcherType": "ALL_KEYS",
                    "negate": false,
                    "userDefinedSegmentMatcherData": null,
                    "whitelistMatcherData": null,
                    "unaryNumericMatcherData": null,
                    "betweenMatcherData": null,
                    "booleanMatcherData": null,
                    "dependencyMatcherData": null,
                    "stringMatcherData": null
                  }
                ]
              },
              "partitions": [
                {
                  "treatment": "on",
                  "size": 0
                },
                {
                  "treatment": "off",
                  "size": 100
                }
              ],
              "label": "default rule"
            }
          ]
        }"#;

    fn in_between_datetime() -> u64 {
        use chrono::prelude::*;
        chrono::Utc.ymd(2016, 4, 25).and_hms(16, 0, 0).timestamp() as u64
    }
    const ATTRIBUTE_NAME: &str = "some_attribute";
    const SOME_KEY: &str = "some_key";
    const UNKNOWN_FEATURE_NAME: &str = "foobar";
}

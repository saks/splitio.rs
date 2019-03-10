use crate::Treatment;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Condition {
    pub label: Option<String>,
    pub condition_type: Option<ConditionType>,
    pub matcher_group: Option<MatcherGroup>,
    pub partitions: Vec<Partition>,
}

impl Condition {
    pub fn is_empty(&self) -> bool {
        self.label.is_some()
            && self.condition_type.is_some()
            && self.matcher_group.is_some()
            && !self.partitions.is_empty()
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum ConditionType {
    Whitelist,
    Rollout,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MatcherGroup {
    combiner: Combiner,
    matchers: Vec<Matcher>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum Combiner {
    And,
    Or,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Matcher {
    matcher_type: MatcherType,
    negate: bool,
    whitelist_matcher_data: Option<WhitelistMatcherData>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MatcherType {
    AllKeys,
    InSegment,
    Whitelist,
    EqualTo,
    GreaterThanOrEqualTo,
    LessThanOrEqualTo,
    Between,
    EqualToSet,
    ContainsAnyOfSet,
    ContainsAllOfSet,
    PartOfSet,
    StartsWith,
    EndsWith,
    ContainsString,
    InSplitTreatment,
    EqualToBoolean,
    MatchesString,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WhitelistMatcherData {
    whitelist: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Partition {
    treatment: String,
    size: u8,
}
